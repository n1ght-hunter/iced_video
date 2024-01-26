//! The player implementation.

use std::{path::PathBuf, sync::Arc};

use ffmpeg::frame::Video;
use futures::{future::OptionFuture, FutureExt};
use playbin_core::{BasicPlayer, IcedImage, PlayerBuilder};
use smol::lock::Mutex;

mod audio;
mod video;

#[cfg(feature = "iced")]
type PlayerMessage<P> = playbin_core::PlayerMessage<P>;

#[cfg(not(feature = "iced"))]
type PlayerMessage<P> = playbin_core::PlayerMessage<P, crate::Frame>;

/// Control commands that can be sent to the player.
#[derive(Clone, Copy, Debug)]
pub enum ControlCommand {
    /// Resume playback.
    Play,
    /// Pause playback.
    Pause,
}

/// The player implementation.
#[derive(Clone, Debug)]
pub struct Player {
    control_sender: Option<smol::channel::Sender<ControlCommand>>,
    demuxer_thread: Option<Arc<std::thread::JoinHandle<()>>>,
    playing: bool,
    // playing_changed_callback: Box<dyn Fn(bool)>,
    event_sender: smol::channel::Sender<PlayerMessage<Self>>,
    player_builder: PlayerBuilder,
}

impl Player {
    /// Create a new player.
    pub fn start(
        player_builder: PlayerBuilder,
    ) -> (Self, smol::channel::Receiver<PlayerMessage<Self>>) {
        let (event_sender, event_receiver) = smol::channel::unbounded();
        let playing = true;
        // playing_changed_callback(playing);

        (
            Self {
                control_sender: None,
                demuxer_thread: None,
                playing,
                // playing_changed_callback: Box::new(playing_changed_callback),
                event_sender,
                player_builder,
            },
            event_receiver,
        )
    }

    fn new_source(&mut self, path: PathBuf) -> Result<(), anyhow::Error> {
        let (control_sender, control_receiver) = smol::channel::unbounded();

        let event_sender = self.event_sender.clone();

        self.control_sender = Some(control_sender);

        let id = self.player_builder.id.clone();

        self.demuxer_thread = Some(Arc::new(std::thread::Builder::new()
            .name("demuxer thread".into())
            .spawn(move || {
                smol::block_on(async move {
                    let mut to_rgba_rescaler: Option<Rescaler> = None;
                    let mut input_context = ffmpeg::format::input(&path).unwrap();

                    let video_stream = input_context
                        .streams()
                        .best(ffmpeg::media::Type::Video)
                        .unwrap();

                    let video_stream_index = video_stream.index();
                    let video_playback_thread = video::VideoPlaybackThread::start(
                        &video_stream,
                        Box::new(move |frame| {
                            let rebuild_rescaler =
                                to_rgba_rescaler.as_ref().map_or(true, |existing_rescaler| {
                                    existing_rescaler.input().format != frame.format()
                                });

                            if rebuild_rescaler {
                                to_rgba_rescaler = Some(rgba_rescaler_for_frame(frame));
                            }

                            let rescaler = to_rgba_rescaler.as_mut().unwrap();

                            let mut rgb_frame = ffmpeg::util::frame::Video::empty();
                            rescaler.run(&frame, &mut rgb_frame).unwrap();

                            #[cfg(feature = "iced")]
                            let frame = crate::Frame(rgb_frame).get_image();
                            
                            #[cfg(not(feature = "iced"))]
                            let frame = crate::Frame(rgb_frame);

                            if let Err(e) = event_sender.try_send(PlayerMessage::Frame(id.clone(),frame)) {
                                println!("Error sending frame: {:?}", e);
                            }
                        }),
                    )
                    .unwrap();

                    let audio_stream = input_context
                        .streams()
                        .best(ffmpeg::media::Type::Audio)
                        .unwrap();
                    let audio_stream_index = audio_stream.index();
                    let audio_playback_thread =
                        audio::AudioPlaybackThread::start(&audio_stream).unwrap();

                    let mut playing = true;

                    // This is sub-optimal, as reading the packets from ffmpeg might be blocking
                    // and the future won't yield for that. So while ffmpeg sits on some blocking
                    // I/O operation, the caller here will also block and we won't end up polling
                    // the control_receiver future further down.
                    let packet_forwarder_impl = async {
                        for (stream, packet) in input_context.packets() {
                            if stream.index() == audio_stream_index {
                                let _ = audio_playback_thread.receive_packet(packet).await;
                            } else if stream.index() == video_stream_index {
                                let _ = video_playback_thread.receive_packet(packet).await;
                            }
                        }
                    }
                    .fuse()
                    .shared();

                    loop {
                        // This is sub-optimal, as reading the packets from ffmpeg might be blocking
                        // and the future won't yield for that. So while ffmpeg sits on some blocking
                        // I/O operation, the caller here will also block and we won't end up polling
                        // the control_receiver future further down.
                        let packet_forwarder: OptionFuture<_> = if playing {
                            Some(packet_forwarder_impl.clone())
                        } else {
                            None
                        }
                        .into();

                        smol::pin!(packet_forwarder);

                        futures::select! {
                            _ = packet_forwarder => {}, // playback finished
                            received_command = control_receiver.recv().fuse() => {
                                match received_command {
                                    Ok(command) => {
                                        video_playback_thread.send_control_message(command).await;
                                        audio_playback_thread.send_control_message(command).await;
                                        match command {
                                            ControlCommand::Play => {
                                                // Continue in the loop, polling the packet forwarder future to forward
                                                // packets
                                                playing = true;
                                            },
                                            ControlCommand::Pause => {
                                                playing = false;
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        // Channel closed -> quit
                                        return;
                                    }
                                }
                            }
                        }
                    }
                })
            })?));

        Ok(())
    }
}

// Work around https://github.com/zmwangx/rust-ffmpeg/issues/102
#[derive(derive_more::Deref, derive_more::DerefMut)]
struct Rescaler(ffmpeg::software::scaling::Context);

#[allow(unsafe_code)]
unsafe impl std::marker::Send for Rescaler {}

fn rgba_rescaler_for_frame(frame: &ffmpeg::util::frame::Video) -> Rescaler {
    Rescaler(
        ffmpeg::software::scaling::Context::get(
            frame.format(),
            frame.width(),
            frame.height(),
            ffmpeg::format::Pixel::RGBA,
            frame.width(),
            frame.height(),
            ffmpeg::software::scaling::Flags::BILINEAR,
        )
        .unwrap(),
    )
}

impl Drop for Player {
    fn drop(&mut self) {
        if let Some(control_sender) = self.control_sender.as_ref() {
            let _ = control_sender.close();
        }
        if let Some(decoder_thread) = self.demuxer_thread.take() {
            Arc::try_unwrap(decoder_thread).unwrap().join().unwrap();
        }
    }
}

impl BasicPlayer for Player {
    type Error = anyhow::Error;

    fn create(player_builder: PlayerBuilder) -> (Self, smol::channel::Receiver<PlayerMessage<Self>>)
    where
        Self: Sized,
    {
        let mut player = Self::start(player_builder.clone());
        if player_builder.auto_start && player_builder.uri.is_some() {
            player.0.set_source(&player_builder.uri.unwrap()).unwrap();
        }
        player
    }

    fn set_source(&mut self, uri: &std::path::PathBuf) -> Result<(), Self::Error> {
        self.new_source(uri.to_owned())
    }

    fn play(&self) {
        if let Some(control_sender) = self.control_sender.as_ref() {
            control_sender.try_send(ControlCommand::Play).unwrap();
        }
    }

    fn pause(&self) {
        if let Some(control_sender) = self.control_sender.as_ref() {
            control_sender.try_send(ControlCommand::Pause).unwrap();
        }
    }

    fn stop(&mut self) {
        if let Some(demuxer_thread) = self.demuxer_thread.take() {
            Arc::try_unwrap(demuxer_thread).unwrap().join().unwrap();
        }
    }

    fn get_source(&self) -> Option<String> {
        todo!()
    }

    fn is_playing(&self) -> bool {
        todo!()
    }
}
