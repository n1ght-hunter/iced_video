use std::sync::Arc;

use gst::prelude::*;

use anyhow::Error;
use derive_more::{Display, Error};
use gst_video::VideoFormat;
use iced::futures::SinkExt;
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
    Connected(VideoPlayer, Option<image::Handle>),
    Disconnected,
    FrameUpdate(Option<image::Handle>),
}

#[derive(Debug)]
enum State {
    NextFrame(futures_channel::mpsc::UnboundedReceiver<image::Handle>),
    Starting(
        VideoPlayer,
        futures_channel::mpsc::UnboundedReceiver<image::Handle>,
    ),
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
pub struct VideoPlayer {
    pub bus: gst::Bus,
    pub frame: Option<image::Handle>,
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

impl VideoPlayer {
    /// Create a new video player from a given video which loads from `uri`.
    ///
    /// If `live` is set then no duration is queried (as this will result in an error and is non-sensical for live streams).
    /// Set `live` if the streaming source is indefinite (e.g. a live stream).
    /// Note that this will cause the duration to be zero.
    pub fn new(uri: &str, live: bool) -> Result<Subscription<VideoEvent>, Error> {
        // Initialize GStreamer
        gst::init()?;

        let pipeline =
            gst::ElementFactory::make("playbin", None).map_err(|_| MissingElement("playbin"))?;

        pipeline.set_property("uri", uri);

        // Create elements that go inside the sink bin
        let videoconvert = gst::ElementFactory::make("videoconvert", None)
            .map_err(|_| MissingElement("videoconvert"))?;

        let scale = gst::ElementFactory::make("videoscale", None)
            .map_err(|_| MissingElement("videoscale"))?;

        let sink = gst::ElementFactory::make("appsink", Some("sink"))
            .map_err(|_| MissingElement("appsink"))?;

        // Create the sink bin, add the elements and link them
        let bin = gst::Bin::new(Some("video-bin"));
        bin.add_many(&[&videoconvert, &scale, &sink])?;
        gst::Element::link_many(&[&videoconvert, &scale, &sink])?;

        // create ghost pad
        let pad = videoconvert
            .static_pad("sink")
            .expect("Failed to get a static pad from equalizer.");
        let ghost_pad = gst::GhostPad::with_target(Some("sink"), &pad).unwrap();
        ghost_pad.set_active(true)?;
        bin.add_pad(&ghost_pad)?;

        pipeline.set_property("video-sink", &bin);

        pipeline.set_state(gst::State::Playing)?;

        // wait for up to 5 seconds until the decoder gets the source capabilities
        pipeline.state(gst::ClockTime::from_seconds(5)).0?;

        let caps = ghost_pad.current_caps().ok_or(MissingCaps("caps"))?;
        let s = caps.structure(0).ok_or(MissingCaps("caps"))?;

        let width = s.get::<i32>("width").map_err(|_| MissingCaps("caps"))?;
        let height = s.get::<i32>("height").map_err(|_| MissingCaps("caps"))?;
        let framerate = s
            .get::<gst::Fraction>("framerate")
            .map_err(|_| MissingCaps("caps"))?;


        // if live getting the duration doesn't make sense 
        let duration = if !live {
            std::time::Duration::from_nanos(
                pipeline
                    .query_duration::<gst::ClockTime>()
                    .ok_or(MissingCaps("caps"))?
                    .nseconds(),
            )
        } else {
            std::time::Duration::from_secs(0)
        };

        let app_sink = sink
            .clone()
            .dynamic_cast::<gst_app::AppSink>()
            .expect("Sink element is expected to be an appsink!");

        app_sink.set_property("emit-signals", true);

        app_sink.set_caps(Some(
            &gst_video::VideoCapsBuilder::new()
                .format(VideoFormat::Bgra)
                .pixel_aspect_ratio(gst::Fraction::new(1, 1))
                .build(),
        ));

        // create channel for sending video frames down
        let (sender, receiver) = futures_channel::mpsc::unbounded::<image::Handle>();

        // callback for video sink
        // creates then sends video handle to subscription 
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

                    if !sender.is_closed() {
                        sender
                            .unbounded_send(image::Handle::from_pixels(
                                width as u32,
                                height as u32,
                                map.as_slice().to_owned(),
                            ))
                            .expect("Failed to send");
                    }

                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );
        let video_player = VideoPlayer {
            bus: pipeline
                .bus()
                .expect("Pipeline without bus. Shouldn't happen!"),
            source: pipeline,
            frame: None,
            // receiver: Arc::new(receiver),
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
                        let (item, stream) = stream.into_future().await;

                        (
                            Some(VideoEvent::Connected(video_player, item)),
                            State::NextFrame(stream),
                        )
                    }
                    State::NextFrame(stream) => {
                        let (item, stream) = stream.into_future().await;

                        (
                            Some(VideoEvent::FrameUpdate(item)),
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
        self.source
            .seek_simple(gst::SeekFlags::FLUSH, position)?;
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

    /// Restarts a stream; seeks to the first frame and unpauses, sets the `eos` flag to false.
    pub fn restart_stream(&mut self) -> Result<(), Error> {
        self.is_eos = false;
        self.set_paused(false);
        // self.seek(0)?;
        Ok(())
    }
}
