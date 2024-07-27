use gst::{
    event::Seek,
    prelude::{ElementExtManual, ObjectExt},
    Element, SeekFlags, SeekType,
};

/// Send a seek event to the pipeline
pub fn send_seek_event(pipeline: &Element, rate: f64) -> Result<(), &'static str> {
    // Obtain the current position, needed for the seek event
    let position = match pipeline.query_position::<gst::ClockTime>() {
        Some(pos) => pos,
        None => {
            return Err("Failed to obtain current position");
        }
    };

    // Create the seek event
    let seek_event = if rate > 0. {
        Seek::new(
            rate,
            SeekFlags::FLUSH | SeekFlags::ACCURATE,
            SeekType::Set,
            position,
            SeekType::End,
            gst::ClockTime::ZERO,
        )
    } else {
        Seek::new(
            rate,
            SeekFlags::FLUSH | SeekFlags::ACCURATE,
            SeekType::Set,
            position,
            SeekType::Set,
            position,
        )
    };

    // If we have not done so, obtain the sink through which we will send the seek events
    if let Some(video_sink) = pipeline.property::<Option<Element>>("video-sink") {
        // Send the event
        match video_sink.send_event(seek_event) {
            true => Ok(()),
            false => Err("Failed to send seek event to the sink"),
        }
    } else {
        Err("Failed to update rate")
    }
}

