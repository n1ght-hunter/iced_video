mod unsafe_functions;
use glib::{Cast, ObjectExt};
use gst::{
    prelude::{ElementExtManual, GstBinExtManual},
    traits::{ElementExt, PadExt},
};
use log::{debug, info};
use unsafe_functions::is_initialized;

use crate::helpers::player_backend::PlayerBackend;

/// A gstreamer backend for the player.
#[derive(Debug)]
pub struct GstreamerBackend {
    source: gst::Element,
    bin: gst::Bin,
    ghost_pad: gst::GhostPad,

    settings: crate::helpers::player_backend::VideoSettings,

    video_details: Option<VideoDetails>,
}

/// stores some details about the video.
#[derive(Clone, Debug)]
pub struct VideoDetails {
    width: i32,
    height: i32,
    framerate: f64,
}

#[derive(Debug)]
pub enum GstreamerError {
    Glib(glib::Error),
    MissingElement(&'static str),
    GstBoolError(glib::BoolError),
    TypeMismatch(gst::structure::GetError<glib::value::ValueTypeMismatchError>),
    CustomError(&'static str),
}

impl PlayerBackend for GstreamerBackend {
    type Error = GstreamerError;

    fn new<C, F>(
        video_settings: crate::helpers::player_backend::VideoSettings,
        frame_callback: Option<C>,
        message_callback: Option<F>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        C: FnMut(&gst_app::AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
        F: FnMut(&gst::Bus, &gst::Message) -> gst::prelude::Continue + Send + 'static,
    {
        info!("Initializing Player");

        if is_initialized() {
            debug!("Initialize GStreamer");
            gst::init().map_err(GstreamerError::Glib)?;
        }

        let source = gst::ElementFactory::make("playbin")
            .build()
            .map_err(|_| GstreamerError::MissingElement("playbin"))?;

        let video_convert = gst::ElementFactory::make("videoconvert")
            .build()
            .map_err(|_| GstreamerError::MissingElement("videoconvert"))?;

        let scale = gst::ElementFactory::make("videoscale")
            .build()
            .map_err(|_| GstreamerError::MissingElement("videoscale"))?;

        let app_sink = gst::ElementFactory::make("appsink")
            .name("sink")
            .build()
            .map_err(|_| GstreamerError::MissingElement("appsink"))?
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
        bin.add_many(&[&video_convert, &scale, app_sink.as_ref()])
            .map_err(GstreamerError::GstBoolError)?;
        gst::Element::link_many(&[&video_convert, &scale, app_sink.as_ref()])
            .map_err(GstreamerError::GstBoolError)?;

        if let Some(frame_callback) = frame_callback {
            // callback for video sink
            // creates then sends video handle to subscription
            app_sink.set_callbacks(
                gst_app::AppSinkCallbacks::builder()
                    .new_sample(frame_callback)
                    .build(),
            );
        }

        debug!("Create ghost pad");
        let pad = video_convert
            .static_pad("sink")
            .ok_or(GstreamerError::MissingElement("no ghost pad"))?;
        let ghost_pad =
            gst::GhostPad::with_target(Some("sink"), &pad).map_err(GstreamerError::GstBoolError)?;
        ghost_pad
            .set_active(true)
            .map_err(GstreamerError::GstBoolError)?;
        bin.add_pad(&ghost_pad)
            .map_err(GstreamerError::GstBoolError)?;

        let mut backend = GstreamerBackend {
            source,
            bin,
            ghost_pad,
            settings: video_settings,
            video_details: None,
        };

        if let Some(url) = backend.settings.uri.clone() {
            backend.set_source(&url)?;
        };

        info!("player initialized");
        Ok(backend)
    }

    fn set_source(&mut self, uri: &str) -> Result<(), Self::Error> {
        info!("Setting source to {}", uri);
        self.source.set_property("uri", &uri);

        self.source.set_property("video-sink", &self.bin);

        debug!("Waiting for decoder to get source capabilities");
        // wait for up to 5 seconds until the decoder gets the source capabilities
        let _ = self
            .source
            .state(gst::ClockTime::from_seconds(5))
            .0
            .map_err(|_| GstreamerError::CustomError("state change error"))?;
        let caps = self
            .ghost_pad
            .current_caps()
            .ok_or(GstreamerError::MissingElement("ghost pad"))?;

        let s = caps
            .structure(0)
            .ok_or(GstreamerError::MissingElement("caps"))?;

        let framerate = s
            .get::<gst::Fraction>("framerate")
            .map_err(GstreamerError::TypeMismatch)?;

        self.video_details = Some(VideoDetails {
            width: s
                .get::<i32>("width")
                .map_err(GstreamerError::TypeMismatch)?,
            height: s
                .get::<i32>("height")
                .map_err(GstreamerError::TypeMismatch)?,
            framerate: framerate.numer() as f64 / framerate.denom() as f64,
        });

        Ok(())
    }

    fn get_source(&self) -> Option<String> {
        self.source.property("uri")
    }

    fn set_volume(&mut self, volume: f64) {
        debug!("volume set to: {}", volume);
        self.source.set_property("volume", &volume);
    }

    fn get_volume(&self) -> f64 {
        self.source.property("volume")
    }

    fn set_mute(&mut self, mute: bool) {
        debug!("muted set to: {}", mute);
        self.source.set_property("mute", &mute);
    }

    fn get_mute(&self) -> bool {
        self.source.property("mute")
    }

    fn set_looping(&mut self, looping: bool) {
        todo!()
    }

    fn get_looping(&self) -> bool {
        todo!()
    }

    fn set_paused(&mut self, paused: bool) -> Result<(), Self::Error> {
        debug!("set paused state to: {}", paused);
        let _ = self
            .source
            .set_state(if paused {
                gst::State::Paused
            } else {
                gst::State::Playing
            })
            .map_err(|_| GstreamerError::CustomError("Element failed to change its state"))?;

        Ok(())
    }

    fn get_paused(&self) -> bool {
        match self.source.state(None).1 {
            gst::State::Playing => true,
            _ => false,
        }
    }

    fn seek(&mut self, position: u64) -> Result<(), Self::Error> {
        debug!("seeking to: {}", position);
        self.source
            .seek_simple(gst::SeekFlags::FLUSH, position * gst::ClockTime::SECOND)
            .map_err(GstreamerError::GstBoolError)?;
        Ok(())
    }

    fn get_position(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(
            self.source
                .query_position::<gst::ClockTime>()
                .map_or(0, |pos| pos.nseconds()),
        )
    }

    fn get_duration(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(
            self.source
                .query_duration::<gst::ClockTime>()
                .map_or(0, |pos| pos.nseconds()),
        )
    }

    fn get_bus(&self) -> &gst::Bus {
        todo!()
    }

    fn exit(&mut self) -> Result<(), Self::Error> {
        debug!("exiting");
        let _ = self.source.send_event(gst::event::Eos::new());
        Ok(())
    }

    fn restart_stream(&mut self) -> Result<(), Self::Error> {
        self.set_paused(false)?;
        self.seek(0)?;
        Ok(())
    }
}
