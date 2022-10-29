use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};

use gst::prelude::*;

use anyhow::Error;
use derive_more::{Display, Error};
use gst_video::VideoFormat;
use iced::futures::sink::SinkExt;
use iced::futures::StreamExt;

use iced::{subscription, Subscription};
use iced_native::{image, widget::Image};

/// Position in the media.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Position {
    /// Position based on time.
    ///
    /// Not the most accurate format for videos.
    Time(std::time::Duration),
    /// Position based on nth frame.
    Frame(u64),
}

impl From<Position> for gst::GenericFormattedValue {
    fn from(pos: Position) -> Self {
        match pos {
            Position::Time(t) => gst::ClockTime::from_nseconds(t.as_nanos() as _).into(),
            Position::Frame(f) => gst::format::Default::from_u64(f).into(),
        }
    }
}

impl From<std::time::Duration> for Position {
    fn from(t: std::time::Duration) -> Self {
        Position::Time(t)
    }
}

impl From<u64> for Position {
    fn from(f: u64) -> Self {
        Position::Frame(f)
    }
}

#[derive(Clone, Debug)]
pub enum VideoEvent {
    Connected(VideoPlayer, Option<GSTChannel>),
    Disconnected,
    FrameUpdate(Option<GSTChannel>),
}

#[derive(Debug)]
enum State {
    NextFrame(Receiver<GSTChannel>),
    Starting(VideoPlayer, Receiver<GSTChannel>),
    // Connected(futures_channel::mpsc::UnboundedReceiver<image::Handle>),
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing Caps {}", _0)]
struct MissingCaps(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

#[derive(Clone, Debug)]
pub enum GSTChannel {
    Image(image::Handle),
    Message(GSTMessage),
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

#[derive(Clone, Debug)]
pub struct VideoPlayer {
    pub bus: gst::Bus,
    pub source: gst::Element,

    pub width: i32,
    pub height: i32,
    pub framerate: f64,
    pub duration: std::time::Duration,

    pub paused: bool,
    pub muted: bool,
    pub looping: bool,
    pub is_eos: bool,
    pub restart_stream: bool,
}

impl Drop for VideoPlayer {
    fn drop(&mut self) {
        self.source
            .set_state(gst::State::Null)
            .expect("failed to set state");
    }
}

impl VideoPlayer {
    /// Create a new video player from a given video which loads from `uri`.
    ///
    /// If `live` is set then no duration is queried (as this will result in an error and is non-sensical for live streams).
    /// Set `live` if the streaming source is indefinite (e.g. a live stream).
    /// Note that this will cause the duration to be zero.
    pub fn new(uri: &str, live: bool) -> Result<Subscription<VideoEvent>, Error> {
        // Initialize GStreamer
        gst::init()?;

        let source =
            gst::ElementFactory::make("playbin", None).map_err(|_| MissingElement("playbin"))?;

        source.set_property("uri", uri);

        // Create elements that go inside the sink bin
        let videoconvert = gst::ElementFactory::make("videoconvert", None)
            .map_err(|_| MissingElement("videoconvert"))?;

        let scale = gst::ElementFactory::make("videoscale", None)
            .map_err(|_| MissingElement("videoscale"))?;

        let app_sink = gst::ElementFactory::make("appsink", Some("sink"))
            .map_err(|_| MissingElement("appsink"))?
            .dynamic_cast::<gst_app::AppSink>()
            .unwrap();

        app_sink.set_property("emit-signals", true);

        app_sink.set_caps(Some(
            &gst_video::VideoCapsBuilder::new()
                .format(VideoFormat::Bgra)
                .pixel_aspect_ratio(gst::Fraction::new(1, 1))
                .build(),
        ));

        // Create the sink bin, add the elements and link them
        let bin = gst::Bin::new(Some("video-bin"));
        bin.add_many(&[&videoconvert, &scale, app_sink.as_ref()])?;
        gst::Element::link_many(&[&videoconvert, &scale, app_sink.as_ref()])?;

        // create ghost pad
        let pad = videoconvert
            .static_pad("sink")
            .expect("Failed to get a static pad from equalizer.");
        let ghost_pad = gst::GhostPad::with_target(Some("sink"), &pad).unwrap();
        ghost_pad.set_active(true)?;
        bin.add_pad(&ghost_pad)?;

        source.set_property("video-sink", &bin);

        source.set_state(gst::State::Playing)?;

        // wait for up to 5 seconds until the decoder gets the source capabilities
        source.state(gst::ClockTime::from_seconds(5)).0?;

        let caps = ghost_pad.current_caps().ok_or(MissingCaps("caps"))?;
        let s = caps.structure(0).ok_or(MissingCaps("caps"))?;

        let width = s.get::<i32>("width").map_err(|_| MissingCaps("caps"))?;
        let height = s.get::<i32>("height").map_err(|_| MissingCaps("caps"))?;
        let framerate = s
            .get::<gst::Fraction>("framerate")
            .map_err(|_| MissingCaps("caps"))?;

        // // if live getting the duration doesn't make sense
        let duration = if !live {
            std::time::Duration::from_nanos(
                source
                    .query_duration::<gst::ClockTime>()
                    .ok_or(MissingCaps("caps"))?
                    .nseconds(),
            )
        } else {
            std::time::Duration::from_secs(0)
        };

        // // create channel for sending video frames down
        let (sender, receiver) = mpsc::channel::<GSTChannel>();
        // let (mut sender, receiver) = futures_channel::mpsc::channel::<image::Handle>(100);

        let sender1 = sender.clone();

        // // callback for video sink
        // // creates then sends video handle to subscription
        app_sink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |sink| {
                    let sample = sink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                    let buffer = sample.buffer().ok_or(gst::FlowError::Error)?;
                    let map = buffer.map_readable().map_err(|_| gst::FlowError::Error)?;

                    let pad = sink.static_pad("sink").ok_or(gst::FlowError::Error)?;

                    let caps = pad.current_caps().ok_or(gst::FlowError::Error)?;
                    let s = caps.structure(0).ok_or(gst::FlowError::Error)?;
                    let width = s.get::<i32>("width").map_err(|_| gst::FlowError::Error)?;
                    let height = s.get::<i32>("height").map_err(|_| gst::FlowError::Error)?;

                    sender
                        .send(GSTChannel::Image(image::Handle::from_pixels(
                            width as u32,
                            height as u32,
                            map.as_slice().to_owned(),
                        )))
                        .map_err(|_| gst::FlowError::Error)?;

                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );

        let bus = source
            .bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        bus.add_watch(move |_bus, msg| {
            println!("event");

            let view = msg.view();

            let message = match view {
                gst::MessageView::Eos(_) => GSTMessage::Eos,
                gst::MessageView::Error(_) => GSTMessage::Error,
                gst::MessageView::Warning(_) => GSTMessage::Warning,
                gst::MessageView::Info(_) => GSTMessage::Info,
                gst::MessageView::Tag(_) => GSTMessage::Tag,
                gst::MessageView::Buffering(_) => GSTMessage::Buffering,
                gst::MessageView::StateChanged(_) => GSTMessage::StateChanged,
                gst::MessageView::StateDirty(_) => GSTMessage::StateDirty,
                gst::MessageView::StepDone(_) => GSTMessage::StepDone,
                gst::MessageView::ClockProvide(_) => GSTMessage::ClockProvide,
                gst::MessageView::ClockLost(_) => GSTMessage::ClockLost,
                gst::MessageView::NewClock(_) => GSTMessage::NewClock,
                gst::MessageView::StructureChange(_) => GSTMessage::StructureChange,
                gst::MessageView::StreamStatus(_) => GSTMessage::StreamStatus,
                gst::MessageView::Application(_) => GSTMessage::Application,
                gst::MessageView::Element(_) => GSTMessage::Element,
                gst::MessageView::SegmentStart(_) => GSTMessage::SegmentStart,
                gst::MessageView::SegmentDone(_) => GSTMessage::SegmentDone,
                gst::MessageView::DurationChanged(_) => GSTMessage::DurationChanged,
                gst::MessageView::Latency(_) => GSTMessage::Latency,
                gst::MessageView::AsyncStart(_) => GSTMessage::AsyncStart,
                gst::MessageView::AsyncDone(_) => GSTMessage::AsyncDone,
                gst::MessageView::RequestState(_) => GSTMessage::RequestState,
                gst::MessageView::StepStart(_) => GSTMessage::StepStart,
                gst::MessageView::Qos(_) => GSTMessage::Qos,
                gst::MessageView::Progress(_) => GSTMessage::Progress,
                gst::MessageView::Toc(_) => GSTMessage::Toc,
                gst::MessageView::ResetTime(_) => GSTMessage::ResetTime,
                gst::MessageView::StreamStart(_) => GSTMessage::StreamStart,
                gst::MessageView::NeedContext(_) => GSTMessage::NeedContext,
                gst::MessageView::HaveContext(_) => GSTMessage::HaveContext,
                gst::MessageView::DeviceAdded(_) => GSTMessage::DeviceAdded,
                gst::MessageView::DeviceRemoved(_) => GSTMessage::DeviceRemoved,
                gst::MessageView::PropertyNotify(_) => GSTMessage::PropertyNotify,
                gst::MessageView::StreamCollection(_) => GSTMessage::StreamCollection,
                gst::MessageView::StreamsSelected(_) => GSTMessage::StreamsSelected,
                gst::MessageView::Redirect(_) => GSTMessage::Redirect,
                gst::MessageView::Other => GSTMessage::Other,
                _ => GSTMessage::Other,
            };

            println!("message: {:?}", message);

            sender1
                .send(GSTChannel::Message(message))
                .expect("unable to send message");

            // Tell the mainloop to continue executing this callback.
            glib::Continue(true)
        })
        .expect("Failed to add bus watch");

        let video_player = VideoPlayer {
            bus,
            source,
            width,
            height,
            framerate: framerate.numer() as f64 / framerate.denom() as f64,
            duration,
            paused: false,
            muted: false,
            looping: false,
            is_eos: false,
            restart_stream: false,
        };

        Ok(subscription::unfold(
            "subscription",
            State::Starting(video_player, receiver),
            |state| async move {
                match state {
                    State::Starting(video_player, stream) => {
                        let item = stream.recv().unwrap();

                        (
                            Some(VideoEvent::Connected(video_player, Some(item))),
                            State::NextFrame(stream),
                        )
                    }
                    State::NextFrame(stream) => {
                        let item = stream.recv().unwrap();

                        (
                            Some(VideoEvent::FrameUpdate(Some(item))),
                            State::NextFrame(stream),
                        )
                    }
                }
            },
        ))
    }

    /// Get the size/resolution of the video as `(width, height)`.
    #[inline(always)]
    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    /// Get the framerate of the video as frames per second.
    #[inline(always)]
    pub fn framerate(&self) -> f64 {
        self.framerate
    }

    /// Set the volume multiplier of the audio.
    /// `0.0` = 0% volume, `1.0` = 100% volume.
    ///
    /// This uses a linear scale, for example `0.5` is perceived as half as loud.
    pub fn set_volume(&mut self, volume: f64) {
        self.source.set_property("volume", &volume);
    }

    /// Set if the audio is muted or not, without changing the volume.
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
        self.source.set_property("mute", &muted);
    }

    /// Get if the audio is muted or not.
    #[inline(always)]
    pub fn muted(&self) -> bool {
        self.muted
    }

    /// Get if the stream ended or not.
    #[inline(always)]
    pub fn eos(&self) -> bool {
        self.is_eos
    }

    /// Get if the media will loop or not.
    #[inline(always)]
    pub fn looping(&self) -> bool {
        self.looping
    }

    /// Set if the media will loop or not.
    #[inline(always)]
    pub fn set_looping(&mut self, looping: bool) {
        self.looping = looping;
    }

    /// Set if the media is paused or not.
    pub fn set_paused(&mut self, paused: bool) {
        self.source
            .set_state(if paused {
                gst::State::Paused
            } else {
                gst::State::Playing
            })
            .unwrap(/* state was changed in ctor; state errors caught there */);
        self.paused = paused;

        // Set restart_stream flag to make the stream restart on the next Message::NextFrame
        if self.is_eos && !paused {
            self.restart_stream = true;
        }
    }

    /// Get if the media is paused or not.
    #[inline(always)]
    pub fn paused(&self) -> bool {
        self.paused
    }

    /// Jumps to a specific position in the media.
    /// The seeking is not perfectly accurate.
    pub fn seek(&mut self, position: impl FormattedValue) -> Result<(), Error> {
        self.source.seek_simple(gst::SeekFlags::FLUSH, position)?;
        Ok(())
    }

    /// Get the current playback position in time.
    pub fn position(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(
            self.source
                .query_position::<gst::ClockTime>()
                .map_or(0, |pos| pos.nseconds()),
        )
        .into()
    }

    /// Get the media duration.
    #[inline(always)]
    pub fn duration(&self) -> std::time::Duration {
        self.duration
    }

    pub fn exit(&mut self) -> Result<(), Error> {
        println!("end stream");
        self.source.send_event(gst::event::Eos::new());
        Ok(())
    }

    /// Restarts a stream; seeks to the first frame and unpauses, sets the `eos` flag to false.
    pub fn restart_stream(&mut self) -> Result<(), Error> {
        self.is_eos = false;
        self.set_paused(false);
        // self.seek(0)?;
        Ok(())
    }
}
