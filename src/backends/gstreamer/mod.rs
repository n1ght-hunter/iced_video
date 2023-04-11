mod error;
mod extra_functions;
pub mod tag_convert;
mod unsafe_functions;
pub use error::GstreamerError;
use gst::{
    glib::{Cast, ObjectExt},
    prelude::{ElementExtManual, GstBinExtManual},
    traits::{ElementExt, PadExt},
    BusSyncReply, FlowError, FlowSuccess,
};
use iced::widget::image;
use log::{debug, info};
use tokio::sync::mpsc;
use unsafe_functions::is_initialized;

use crate::{backends::gstreamer::extra_functions::send_seek_event, PlayerBackend, PlayerBuilder};

/// A gstreamer backend for the player.
#[derive(Debug, Clone)]
pub struct GstreamerBackend {
    playbin: gst::Element,
    bin: gst::Bin,
    ghost_pad: gst::GhostPad,

    settings: crate::PlayerBuilder,

    video_details: Option<VideoDetails>,
    playback_rate: f64,
}

/// stores some details about the video.
#[derive(Clone, Debug)]
pub struct VideoDetails {
    width: i32,
    height: i32,
    framerate: f64,
}

/// The message that is sent to the main thread.
#[derive(Debug, Clone)]
pub enum GstreamerMessage {
    /// The player id and the player.
    Player(String, GstreamerBackend),
    /// The player id and the image.
    Image(String, image::Handle),
    /// The player id and the message.
    Message(String, gst::Message),
}

impl GstreamerBackend {
    /// Creates a gstreamer player.
    pub fn new(
        settings: PlayerBuilder,
    ) -> (GstreamerMessage, mpsc::UnboundedReceiver<GstreamerMessage>) {
        let (sender, receiver) = mpsc::unbounded_channel::<GstreamerMessage>();
        let sender1 = sender.clone();
        let _sender2 = sender.clone();
        let id = settings.id.clone();
        let id1 = settings.id.clone();
        let id2 = settings.id.clone();
        let _id3 = settings.id.clone();
        let player = Self::build_player(
            settings,
            move |sink: &gst_app::AppSink| {
                let sample = sink.pull_sample().map_err(|_| FlowError::Eos)?;
                let buffer = sample.buffer().ok_or(FlowError::Error)?;
                let map = buffer.map_readable().map_err(|_| FlowError::Error)?;

                let pad = sink.static_pad("sink").ok_or(FlowError::Error)?;

                let caps = pad.current_caps().ok_or(FlowError::Error)?;
                let s = caps.structure(0).ok_or(FlowError::Error)?;
                let width = s.get::<i32>("width").map_err(|_| FlowError::Error)?;
                let height = s.get::<i32>("height").map_err(|_| FlowError::Error)?;

                let res = sender.send(GstreamerMessage::Image(
                    id1.clone(),
                    image::Handle::from_pixels(
                        width as u32,
                        height as u32,
                        map.as_slice().to_owned(),
                    ),
                ));

                if res.is_err() {
                    return Err(FlowError::Error);
                }

                Ok(FlowSuccess::Ok)
            },
            move |_, msg| {
                let res = sender1.send(GstreamerMessage::Message(id2.clone(), msg.clone()));

                if res.is_err() {
                    log::error!("Error sending message");
                }
                BusSyncReply::Pass
            },
        )
        .unwrap();

        (GstreamerMessage::Player(id, player), receiver)
    }

    /// Builds the player.
    pub fn build_player<C, F>(
        video_settings: crate::PlayerBuilder,
        frame_callback: C,
        message_callback: F,
    ) -> Result<Self, GstreamerError>
    where
        Self: Sized,
        C: FnMut(&gst_app::AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
        F: Fn(&gst::Bus, &gst::Message) -> BusSyncReply + Send + Sync + 'static,
    {
        info!("Initializing Player");

        if !is_initialized() {
            debug!("Initialize GStreamer");
            gst::init()?;
        }

        let playbin = gst::ElementFactory::make("playbin3").build()?;

        playbin.set_property("instant-uri", true);

        let video_convert = gst::ElementFactory::make("videoconvert").build()?;

        let scale = gst::ElementFactory::make("videoscale").build()?;

        let app_sink = gst::ElementFactory::make("appsink")
            .name("sink")
            .build()?
            .dynamic_cast::<gst_app::AppSink>()
            .expect("unable to cast appsink");

        app_sink.set_property("emit-signals", true);

        app_sink.set_caps(Some(
            &gst_video::VideoCapsBuilder::new()
                .format(gst_video::VideoFormat::Rgba)
                .pixel_aspect_ratio(gst::Fraction::new(1, 1))
                .build(),
        ));

        debug!("Create the sink bin and linking");
        // Create the sink bin, add the elements and link them
        let bin = gst::Bin::new(Some("video-bin"));
        bin.add_many(&[&video_convert, &scale, app_sink.as_ref()])?;
        gst::Element::link_many(&[&video_convert, &scale, app_sink.as_ref()])?;

        // callback for video sink
        // creates then sends video handle to subscription
        app_sink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(frame_callback)
                .build(),
        );
        // callback for bus messages
        // sends messages to subscription
        let bus = playbin
            .bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        let _id = video_settings.id.clone();

        bus.set_sync_handler(message_callback);

        debug!("Create ghost pad");
        let pad = video_convert
            .static_pad("sink")
            .ok_or(GstreamerError::MissingElement("no ghost pad"))?;
        let ghost_pad = gst::GhostPad::with_target(Some("sink"), &pad)?;
        ghost_pad.set_active(true)?;
        bin.add_pad(&ghost_pad)?;

        let mut backend = GstreamerBackend {
            playbin,
            bin,
            ghost_pad,
            settings: video_settings,
            video_details: None,
            playback_rate: 1.0,
        };

        if let Some(url) = backend.settings.uri.clone() {
            backend.set_source(&url)?;
        };

        info!("player initialized");
        Ok(backend)
    }
}

impl PlayerBackend for GstreamerBackend {
    type Error = GstreamerError;

    fn set_source(&mut self, uri: &str) -> Result<(), Self::Error> {
        info!("Setting source to {}", uri);
        self.playbin.set_property("uri", &uri);

        self.playbin.set_property("video-sink", &self.bin);

        let _ = self.playbin.set_state(gst::State::Playing)?;

        debug!("Waiting for decoder to get source capabilities");
        // wait for up to 5 seconds until the decoder gets the source capabilities
        let _ = self.playbin.state(gst::ClockTime::from_seconds(5)).0?;
        let caps = self
            .ghost_pad
            .current_caps()
            .ok_or(GstreamerError::MissingElement("current_caps"))?;

        let s = caps
            .structure(0)
            .ok_or(GstreamerError::MissingElement("caps"))?;

        let framerate = s.get::<gst::Fraction>("framerate")?;

        self.video_details = Some(VideoDetails {
            width: s.get::<i32>("width")?,
            height: s.get::<i32>("height")?,
            framerate: framerate.numer() as f64 / framerate.denom() as f64,
        });

        if !self.settings.auto_start {
            debug!("auto start false setting state to paused");
            let _ = self.playbin.set_state(gst::State::Paused)?;
        }

        Ok(())
    }

    fn get_source(&self) -> Option<String> {
        self.playbin.property("current-uri")
    }

    fn set_volume(&mut self, volume: f64) {
        debug!("volume set to: {}", volume);
        self.playbin.set_property("volume", &volume);
    }

    fn get_volume(&self) -> f64 {
        self.playbin.property("volume")
    }

    fn set_muted(&mut self, mute: bool) {
        debug!("muted set to: {}", mute);
        self.playbin.set_property("mute", &mute);
    }

    fn get_mute(&self) -> bool {
        self.playbin.property("mute")
    }

    fn set_looping(&mut self, _looping: bool) {
        todo!()
    }

    fn get_looping(&self) -> bool {
        todo!()
    }

    fn set_paused(&mut self, paused: bool) -> Result<(), Self::Error> {
        debug!("set paused state to: {}", paused);
        let _ = self
            .playbin
            .set_state(if paused {
                gst::State::Paused
            } else {
                gst::State::Playing
            })
            .map_err(|_| GstreamerError::CustomError("Element failed to change its state"))?;

        Ok(())
    }

    fn get_paused(&self) -> bool {
        match self.playbin.state(None).1 {
            gst::State::Playing => false,
            _ => true,
        }
    }

    fn seek(&mut self, position: u64) -> Result<(), Self::Error> {
        debug!("seeking to: {}", position);
        self.playbin
            .seek_simple(gst::SeekFlags::FLUSH, position * gst::ClockTime::SECOND)?;
        Ok(())
    }

    fn get_position(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(
            self.playbin
                .query_position::<gst::ClockTime>()
                .map_or(0, |pos| pos.nseconds()),
        )
    }

    fn get_duration(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(
            self.playbin
                .query_duration::<gst::ClockTime>()
                .map_or(0, |pos| pos.nseconds()),
        )
    }

    fn get_rate(&self) -> f64 {
        self.playback_rate
    }

    fn next_frame(&mut self) -> Result<(), Self::Error> {
        if let Some(video_sink) = self.playbin.property::<Option<gst::Element>>("video-sink") {
            debug!("Stepping one frame");
            // Send the event
            let step = gst::event::Step::new(
                gst::format::Buffers::ONE,
                self.playback_rate.abs(),
                true,
                false,
            );
            match video_sink.send_event(step) {
                true => Ok(()),
                false => Err("Failed to send seek event to the sink".into()),
            }
        } else {
            Err("No video sink found".into())
        }
    }

    fn previous_frame(&mut self) -> Result<(), Self::Error> {
        if let Some(video_sink) = self.playbin.property::<Option<gst::Element>>("video-sink") {
            debug!("Stepping one frame");
            // Send the event
            let step = gst::event::Step::new(
                gst::format::Buffers::ONE,
                -self.playback_rate.abs(),
                true,
                false,
            );
            match video_sink.send_event(step) {
                true => Ok(()),
                false => Err("Failed to send seek event to the sink".into()),
            }
        } else {
            Err("No video sink found".into())
        }
    }

    fn set_rate(&self, rate: f64) -> Result<(), Self::Error> {
        debug!("set rate to: {}", rate);
        send_seek_event(&self.playbin, rate)?;
        Ok(())
    }

    fn exit(&mut self) -> Result<(), Self::Error> {
        debug!("exiting");
        let _ = self.playbin.send_event(gst::event::Eos::new());
        Ok(())
    }

    fn restart_stream(&mut self) -> Result<(), Self::Error> {
        self.set_paused(false)?;
        self.seek(0)?;
        Ok(())
    }
}
