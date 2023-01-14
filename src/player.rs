use anyhow::Error;
use derive_more::{Display, Error};
pub use gst::{prelude::*, Buffer, Bus, Message};
pub use gst::{
    traits::{ElementExt, PadExt},
    FlowError, FlowSuccess, MessageView,
};
use gst_app::AppSink;
pub use gst_video::VideoFormat;
use log::{debug, info};

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

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing {}", _0)]
struct Missing(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

/// setting when creating a player
#[derive(Clone, Debug)]
pub struct VideoSettings {
    /// start player in play state
    pub auto_start: bool,
    /// if live duration won't work and trying to seek will cause a panic
    pub live: bool,
    /// vdieo uri
    pub uri: Option<String>,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            auto_start: false,
            live: false,
            uri: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VideoPlayer {
    bus: gst::Bus,
    source: gst::Element,

    width: i32,
    height: i32,
    framerate: f64,
    duration: std::time::Duration,
    paused: bool,
    muted: bool,
    looping: bool,
    is_eos: bool,
    restart_stream: bool,
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
    pub fn new<C, F>(
        settings: VideoSettings,
        format: VideoFormat,
        sample_callback: C,
        message_callback: F,
    ) -> Result<VideoPlayer, Error>
    where
        C: FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
        F: FnMut(&Bus, &Message) -> gst::prelude::Continue + Send + 'static,
    {
        info!("Initializing Player");
        debug!("Initialize GStreamer");
        // Initialize GStreamer
        gst::init()?;

        debug!("Initialize playbin");
        // playbin handle most sources and offers easy to impl controls
        let source = if let Some(uri) = settings.uri {
            gst::ElementFactory::make("playbin")
                .property("uri", uri)
                .build()
                .map_err(|_| MissingElement("playbin"))?
        } else {
            gst::ElementFactory::make("playbin")
                .build()
                .map_err(|_| MissingElement("playbin"))?
        };

        // Create elements that go inside the sink bin
        let videoconvert = gst::ElementFactory::make("videoconvert")
            .build()
            .map_err(|_| MissingElement("videoconvert"))?;

        let scale = gst::ElementFactory::make("videoscale")
            .build()
            .map_err(|_| MissingElement("videoscale"))?;

        let app_sink = gst::ElementFactory::make("appsink")
            .name("sink")
            .build()
            .map_err(|_| MissingElement("appsink"))?
            .dynamic_cast::<gst_app::AppSink>()
            .expect("unable to cast appsink");

        app_sink.set_property("emit-signals", true);

        app_sink.set_caps(Some(
            &gst_video::VideoCapsBuilder::new()
                .format(format)
                .pixel_aspect_ratio(gst::Fraction::new(1, 1))
                .build(),
        ));

        debug!("Create the sink bin and linking");
        // Create the sink bin, add the elements and link them
        let bin = gst::Bin::new(Some("video-bin"));
        bin.add_many(&[&videoconvert, &scale, app_sink.as_ref()])?;
        gst::Element::link_many(&[&videoconvert, &scale, app_sink.as_ref()])?;

        // create ghost pad
        let pad = videoconvert
            .static_pad("sink")
            .ok_or(MissingElement("no ghost pad"))?;
        let ghost_pad = gst::GhostPad::with_target(Some("sink"), &pad)?;
        ghost_pad.set_active(true)?;
        bin.add_pad(&ghost_pad)?;

        source.set_property("video-sink", &bin);

        // source.set_state(gst::State::Playing)?;

        // debug!("Waiting for decoder to get source capabilities");
        // wait for up to 5 seconds until the decoder gets the source capabilities
        // source.state(gst::ClockTime::from_seconds(5)).0?;
        let caps = ghost_pad.current_caps().ok_or(Missing("ghost_pad"))?;

        let s = caps.structure(0).ok_or(Missing("caps"))?;

        let width = s.get::<i32>("width").map_err(|_| Missing("width"))?;
        let height = s.get::<i32>("height").map_err(|_| Missing("height"))?;
        let framerate = s
            .get::<gst::Fraction>("framerate")
            .map_err(|_| Missing("framerate"))?;

        // if live getting the duration doesn't make sense
        // let duration = if !settings.live {
        //     debug!("getting duration");
        //     std::time::Duration::from_nanos(
        //         source
        //             .query_duration::<gst::ClockTime>()
        //             .ok_or(Missing("Duration"))?
        //             .nseconds(),
        //     )
        // } else {
        //     debug!("live no duration");
        //     std::time::Duration::from_secs(0)
        // };
        let duration = std::time::Duration::from_secs(0);

        // callback for video sink
        // creates then sends video handle to subscription
        app_sink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(sample_callback)
                .build(),
        );

        let bus = source
            .bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        bus.add_watch(message_callback)
            .expect("Failed to add bus watch");

        if !settings.auto_start {
            debug!("auto start false setting state to paused");
            source.set_state(gst::State::Paused)?;
        }

        let video_player = VideoPlayer {
            bus,
            source,
            width,
            height,
            framerate: framerate.numer() as f64 / framerate.denom() as f64,
            duration,
            paused: if settings.auto_start { false } else { true },
            muted: false,
            looping: false,
            is_eos: false,
            restart_stream: false,
        };
        info!("player initialized");
        Ok(video_player)
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

    #[inline(always)]
    pub fn uri(&self) -> String {
        self.source.property("uri")
    }

    pub fn set_uri(&self, uri: impl Into<String>) {
        self.source.set_property("uri", uri.into());
    }

    /// Set the volume multiplier of the audio.
    /// `0.0` = 0% volume, `1.0` = 100% volume.
    ///
    /// This uses a linear scale, for example `0.5` is perceived as half as loud.
    pub fn set_volume(&mut self, volume: f64) {
        debug!("volume set to: {}", volume);
        self.source.set_property("volume", &volume);
    }

    /// get the volume multiplier of the audio.
    /// `0.0` = 0% volume, `1.0` = 100% volume.
    ///
    /// This uses a linear scale, for example `0.5` is perceived as half as loud.
    pub fn get_volume(&self) -> f64 {
        self.source.property("volume")
    }

    /// Set if the audio is muted or not, without changing the volume.
    pub fn set_muted(&mut self, muted: bool) {
        debug!("muted set to: {}", muted);
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
        debug!("looping set to: {}", looping);
        self.looping = looping;
    }

    /// Set if the media is paused or not.
    pub fn set_paused_state(&mut self, paused: bool) {
        debug!("set paused state to: {}", paused);
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
    pub fn seek(&mut self, position: u64) -> Result<(), Error> {
        debug!("seeking to: {}", position);
        self.source
            .seek_simple(gst::SeekFlags::FLUSH, position * gst::ClockTime::SECOND)?;
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

    #[inline(always)]
    pub fn get_bus(&self) -> &Bus {
        &self.bus
    }

    pub fn exit(&mut self) -> Result<(), Error> {
        debug!("exiting");
        self.source.send_event(gst::event::Eos::new());
        Ok(())
    }

    /// Restarts a stream; seeks to the first frame and unpauses, sets the `eos` flag to false.
    pub fn restart_stream(&mut self) -> Result<(), Error> {
        self.is_eos = false;
        self.set_paused_state(false);
        // self.seek(0)?;
        Ok(())
    }
}
