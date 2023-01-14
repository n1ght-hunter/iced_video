use std::{borrow::Cow, sync::mpsc};

use core::hash::Hash;
use gst::{
    prelude::Continue,
    traits::{ElementExt, PadExt},
    FlowError, FlowSuccess, MessageView,
};
use gst_video::VideoFormat;
use iced::{subscription, widget::image};

use crate::player::{VideoPlayer, VideoSettings};

#[derive(Clone, Debug)]
pub enum SubMSG {
    Player(String, VideoPlayer),
    Image(String, image::Handle),
    Message(String, GSTMessage),
}

#[derive(Debug)]
enum PlayerSubscription {
    Starting(String, VideoSettings),
    Next(mpsc::Receiver<SubMSG>),
}

pub fn video_subscription(id: impl Into<String>, settings: VideoSettings) -> iced::Subscription<SubMSG> {
    let id: String = id.into();
    subscription::unfold(
        id.clone(),
        PlayerSubscription::Starting(id, settings),
        |state| async move {
            match state {
                PlayerSubscription::Starting(id, settings) => {
                    let (sender, receiver) = mpsc::channel::<SubMSG>();
                    let sender1 = sender.clone();
                    let id1 = id.clone();
                    let id2 = id.clone();
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
                                .send(SubMSG::Image(
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
                            let view = msg.view();

                            let message = match view {
                                MessageView::Eos(_) => GSTMessage::Eos,
                                MessageView::Error(_) => GSTMessage::Error,
                                MessageView::Warning(_) => GSTMessage::Warning,
                                MessageView::Info(_) => GSTMessage::Info,
                                MessageView::Tag(_) => GSTMessage::Tag,
                                MessageView::Buffering(_) => GSTMessage::Buffering,
                                MessageView::StateChanged(_) => GSTMessage::StateChanged,
                                MessageView::StateDirty(_) => GSTMessage::StateDirty,
                                MessageView::StepDone(_) => GSTMessage::StepDone,
                                MessageView::ClockProvide(_) => GSTMessage::ClockProvide,
                                MessageView::ClockLost(_) => GSTMessage::ClockLost,
                                MessageView::NewClock(_) => GSTMessage::NewClock,
                                MessageView::StructureChange(_) => GSTMessage::StructureChange,
                                MessageView::StreamStatus(_) => GSTMessage::StreamStatus,
                                MessageView::Application(_) => GSTMessage::Application,
                                MessageView::Element(_) => GSTMessage::Element,
                                MessageView::SegmentStart(_) => GSTMessage::SegmentStart,
                                MessageView::SegmentDone(_) => GSTMessage::SegmentDone,
                                MessageView::DurationChanged(_) => GSTMessage::DurationChanged,
                                MessageView::Latency(_) => GSTMessage::Latency,
                                MessageView::AsyncStart(_) => GSTMessage::AsyncStart,
                                MessageView::AsyncDone(_) => GSTMessage::AsyncDone,
                                MessageView::RequestState(_) => GSTMessage::RequestState,
                                MessageView::StepStart(_) => GSTMessage::StepStart,
                                MessageView::Qos(_) => GSTMessage::Qos,
                                MessageView::Progress(_) => GSTMessage::Progress,
                                MessageView::Toc(_) => GSTMessage::Toc,
                                MessageView::ResetTime(_) => GSTMessage::ResetTime,
                                MessageView::StreamStart(_) => GSTMessage::StreamStart,
                                MessageView::NeedContext(_) => GSTMessage::NeedContext,
                                MessageView::HaveContext(_) => GSTMessage::HaveContext,
                                MessageView::DeviceAdded(_) => GSTMessage::DeviceAdded,
                                MessageView::DeviceRemoved(_) => GSTMessage::DeviceRemoved,
                                MessageView::PropertyNotify(_) => GSTMessage::PropertyNotify,
                                MessageView::StreamCollection(_) => GSTMessage::StreamCollection,
                                MessageView::StreamsSelected(_) => GSTMessage::StreamsSelected,
                                MessageView::Redirect(_) => GSTMessage::Redirect,
                                MessageView::Other => GSTMessage::Other,
                                _ => GSTMessage::Other,
                            };

                            sender1
                                .send(SubMSG::Message(id2.clone(), message))
                                .expect("unable to send message");

                            // Tell the mainloop to continue executing this callback.
                            Continue(true)
                        },
                    )
                    .unwrap();
                    (
                        Some(SubMSG::Player(id, player)),
                        PlayerSubscription::Next(receiver),
                    )
                }
                PlayerSubscription::Next(stream) => {
                    let item = stream.recv().unwrap();
                    (Some(item), PlayerSubscription::Next(stream))
                }
            }
        },
    )
}

#[derive(Clone, Debug)]
pub enum GSTMessage {
    Eos,
    Error,
    Warning,
    Info,
    Tag,
    Buffering,
    StateChanged,
    StateDirty,
    StepDone,
    ClockProvide,
    ClockLost,
    NewClock,
    StructureChange,
    StreamStatus,
    Application,
    Element,
    SegmentStart,
    SegmentDone,
    DurationChanged,
    Latency,
    AsyncStart,
    AsyncDone,
    RequestState,
    StepStart,
    Qos,
    Progress,
    Toc,
    ResetTime,
    StreamStart,
    NeedContext,
    HaveContext,
    DeviceAdded,
    DeviceRemoved,
    PropertyNotify,
    StreamCollection,
    StreamsSelected,
    Redirect,
    Other,
}
