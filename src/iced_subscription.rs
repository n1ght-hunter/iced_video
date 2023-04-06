//! This module contains the subscription for the video player. this will update the frame with the new video frame.

use gst::{
    prelude::Continue,
    traits::{ElementExt, PadExt},
    FlowError, FlowSuccess, MessageView,
};
use gst_video::VideoFormat;
use iced::{subscription, widget::image};
use tokio::sync::mpsc;

use crate::{player::VideoPlayer, video_settings::VideoSettings};


/// The message that is sent to the main thread.
#[derive(Clone, Debug)]
pub enum PlayerMessage {
    /// The player id and the player.
    Player(String, VideoPlayer),
    /// The player id and the image.
    Image(String, image::Handle),
    /// The player id and the message.
    Message(String, gst::Message),
}

/// The Subscription state.
#[derive(Debug)]
enum PlayerSubscription {
    Starting(VideoSettings),
    Next(mpsc::Receiver<PlayerMessage>),
}

/// The subscription for the video player.
pub fn video_subscription(settings: VideoSettings) -> iced::Subscription<PlayerMessage> {
    subscription::unfold(
        settings.id.clone(),
        PlayerSubscription::Starting(settings),
        |state| async move {
            match state {
                PlayerSubscription::Starting(settings) => {
                    let (sender, receiver) = mpsc::channel::<PlayerMessage>(20);
                    let sender1 = sender.clone();
                    let id = settings.id.clone();
                    let id1 = settings.id.clone();
                    let id2 = settings.id.clone();
                    let player = VideoPlayer::new(
                        settings,
                        VideoFormat::Rgba,
                        move |sink| {
                            let sample = sink.pull_sample().map_err(|_| FlowError::Eos)?;
                            let buffer = sample.buffer().ok_or(FlowError::Error)?;
                            let map = buffer.map_readable().map_err(|_| FlowError::Error)?;

                            let pad = sink.static_pad("sink").ok_or(FlowError::Error)?;

                            let caps = pad.current_caps().ok_or(FlowError::Error)?;
                            let s = caps.structure(0).ok_or(FlowError::Error)?;
                            let width = s.get::<i32>("width").map_err(|_| FlowError::Error)?;
                            let height = s.get::<i32>("height").map_err(|_| FlowError::Error)?;

                            sender
                                .blocking_send(PlayerMessage::Image(
                                    id1.clone(),
                                    image::Handle::from_pixels(
                                        width as u32,
                                        height as u32,
                                        map.as_slice().to_owned(),
                                    ),
                                ))
                                .expect("unable to send");

                            Ok(FlowSuccess::Ok)
                        },
                        move |_bus, msg| {
                            sender1
                                .blocking_send(PlayerMessage::Message(id2.clone(), msg.copy()))
                                .expect("unable to send message");

                            // Tell the mainloop to continue executing this callback.
                            Continue(true)
                        },
                    )
                    .unwrap();
                    (
                        Some(PlayerMessage::Player(id, player)),
                        PlayerSubscription::Next(receiver),
                    )
                }
                PlayerSubscription::Next(mut stream) => {
                    let item = stream.recv().await.unwrap();
                    (Some(item), PlayerSubscription::Next(stream))
                }
            }
        },
    )
}
