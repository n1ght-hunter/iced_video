//! Convert GStreamer tags to Rust tags
use gst::TagList;

/// convert glib types to rust types
#[derive(Debug, Clone)]
pub enum GStreamerTagTypes {
    /// glib char array
    GCharArray(String),
    /// glib uint
    GUint(u32),
    /// glib gst date time
    GstDateTime(String),
    /// unknown type
    Unknown(String),
}

impl TryInto<String> for GStreamerTagTypes {
    type Error = &'static str;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            GStreamerTagTypes::GCharArray(value) => Ok(value),
            GStreamerTagTypes::GUint(_value) => Err("error wrong type"),
            GStreamerTagTypes::GstDateTime(value) => Ok(value),
            GStreamerTagTypes::Unknown(value) => Ok(value),
        }
    }
}

impl TryInto<u32> for GStreamerTagTypes {
    type Error = &'static str;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            GStreamerTagTypes::GCharArray(_value) => Err("error wrong type"),
            GStreamerTagTypes::GUint(value) => Ok(value),
            GStreamerTagTypes::GstDateTime(_value) => Err("error wrong type"),
            GStreamerTagTypes::Unknown(_value) => Err("error wrong type"),
        }
    }
}

/// Rust tags
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum Tag {
    AudioCodec(String),
    MaximumBitrate(u32),
    Bitrate(u32),
    Language(String),
    DateTime(String),
    Title(String),
    Comment(String),
    Encoder(String),
    ContainerFormat(String),
    VideoCodec(String),
    MinimumBitrate(u32),
    Unknown(String, GStreamerTagTypes),
}

impl From<gst::glib::SendValue> for GStreamerTagTypes {
    fn from(value: gst::glib::SendValue) -> Self {
        match value.type_().name() {
            "gchararray" => {
                let value = value.get::<String>().unwrap();
                GStreamerTagTypes::GCharArray(value)
            }
            "guint" => {
                let value = value.get::<u32>().unwrap();
                GStreamerTagTypes::GUint(value)
            }
            "GstDateTime" => {
                let value = value.get::<gst::DateTime>().unwrap();
                GStreamerTagTypes::GstDateTime(value.to_iso8601_string().unwrap().to_string())
            }
            _ => {
                return GStreamerTagTypes::Unknown(value.type_().name().to_string());
            }
        }
    }
}

/// convert gstreamer tags to rust tags
pub trait TaglistToTags {
    /// convert gstreamer tags to rust tags
    fn to_rust_tags(&self) -> Vec<(String, Tag)>;
}

impl TaglistToTags for TagList {
    fn to_rust_tags(&self) -> Vec<(String, Tag)> {
        // let asdf = self.get::<gst::tags::Title>().map(|x| x.get().to_string());

        self.iter()
            .map(|(name, value)| {
                let value = GStreamerTagTypes::from(value);
                let name = name.to_string();
                let tag = match name.as_str() {
                    "audio-codec" => Tag::AudioCodec(value.try_into().unwrap()),
                    "maximum-bitrate" => Tag::MaximumBitrate(value.try_into().unwrap()),
                    "bitrate" => Tag::Bitrate(value.try_into().unwrap()),
                    "language-code" => Tag::Language(value.try_into().unwrap()),
                    "datetime" => Tag::DateTime(value.try_into().unwrap()),
                    "title" => Tag::Title(value.try_into().unwrap()),
                    "comment" => Tag::Comment(value.try_into().unwrap()),
                    "encoder" => Tag::Encoder(value.try_into().unwrap()),
                    "container-format" => Tag::ContainerFormat(value.try_into().unwrap()),
                    "video-codec" => Tag::VideoCodec(value.try_into().unwrap()),
                    "minimum-bitrate" => Tag::MinimumBitrate(value.try_into().unwrap()),
                    _ => Tag::Unknown(name.to_string(), value),
                };

                (name, tag)
            })
            .collect()
    }
}

// audio-codec: (gchararray) "MPEG-4 AAC audio"
// maximum-bitrate: (guint) 224000
// bitrate: (guint) 224000
// language-code: (gchararray) "en"
// datetime: (GstDateTime) ((GstDateTime*) 00000286AFA90C80)
// title: (gchararray) "Scream.2022.1080p.WEBRip.x264-RARBG"
// comment: (gchararray) "Scream.2022.1080p.WEBRip.x264-RARBG"
// encoder: (gchararray) "Lavf58.20.100"
// container-format: (gchararray) "ISO MP4/M4A"
// audio-codec: (gchararray) "MPEG-4 AAC"
// maximum-bitrate: (guint) 224000
// bitrate: (guint) 224000
// language-code: (gchararray) "en"
// video-codec: (gchararray) "H.264 (High Profile)"
// bitrate: (guint) 2497185
// datetime: (GstDateTime) ((GstDateTime*) 00000286AFA90C80)
// title: (gchararray) "Scream.2022.1080p.WEBRip.x264-RARBG"
// comment: (gchararray) "Scream.2022.1080p.WEBRip.x264-RARBG"
// encoder: (gchararray) "Lavf58.20.100"
// container-format: (gchararray) "ISO MP4/M4A"
// audio-codec: (gchararray) "MPEG-4 AAC"
// maximum-bitrate: (guint) 224000
// bitrate: (guint) 224000
// language-code: (gchararray) "en"
// minimum-bitrate: (guint) 223875
// audio-codec: (gchararray) "MPEG-4 AAC"
// maximum-bitrate: (guint) 224000
// bitrate: (guint) 224000
// language-code: (gchararray) "en"
// minimum-bitrate: (guint) 223500
// video-codec: (gchararray) "H.264 (High Profile)"
// bitrate: (guint) 2497185
// minimum-bitrate: (guint) 3799716
// maximum-bitrate: (guint) 3799716
// video-codec: (gchararray) "H.264 (High Profile)"
// bitrate: (guint) 2497185
// minimum-bitrate: (guint) 1198800
// maximum-bitrate: (guint) 3799716
// video-codec: (gchararray) "H.264 (High Profile)"
// bitrate: (guint) 2497185
// minimum-bitrate: (guint) 801517
// maximum-bitrate: (guint) 3799716
// audio-codec: (gchararray) "MPEG-4 AAC"
// maximum-bitrate: (guint) 224000
// bitrate: (guint) 224000
// language-code: (gchararray) "en"
// minimum-bitrate: (guint) 223125
// video-codec: (gchararray) "H.264 (High Profile)"
// bitrate: (guint) 2497185
// minimum-bitrate: (guint) 490309
// maximum-bitrate: (guint) 3799716
// audio-codec: (gchararray) "MPEG-4 AAC"
// maximum-bitrate: (guint) 224000
// bitrate: (guint) 224000
// language-code: (gchararray) "en"
// minimum-bitrate: (guint) 220125
// video-codec: (gchararray) "H.264 (High Profile)"
// bitrate: (guint) 2497185
// minimum-bitrate: (guint) 490309
// maximum-bitrate: (guint) 4599316
// audio-codec: (gchararray) "MPEG-4 AAC"
// maximum-bitrate: (guint) 224000
// bitrate: (guint) 224000
// language-code: (gchararray) "en"
// minimum-bitrate: (guint) 216374
// audio-codec: (gchararray) "MPEG-4 AAC"
// maximum-bitrate: (guint) 224000
// bitrate: (guint) 224000
// language-code: (gchararray) "en"
// minimum-bitrate: (guint) 212624
// video-codec: (gchararray) "H.264 (High Profile)"
// bitrate: (guint) 2497185
// minimum-bitrate: (guint) 468491
// maximum-bitrate: (guint) 4599316
// audio-codec: (gchararray) "MPEG-4 AAC"
// maximum-bitrate: (guint) 224000
// bitrate: (guint) 224000
// language-code: (gchararray) "en"
// minimum-bitrate: (guint) 211124
// video-codec: (gchararray) "H.264 (High Profile)"
// bitrate: (guint) 2497185
// minimum-bitrate: (guint) 383855
// maximum-bitrate: (guint) 4599316
// video-codec: (gchararray) "H.264 (High Profile)"
// bitrate: (guint) 2497185
// minimum-bitrate: (guint) 362037
// maximum-bitrate: (guint) 4599316
// audio-codec: (gchararray) "MPEG-4 AAC"
// maximum-bitrate: (guint) 224000
// bitrate: (guint) 224000
// language-code: (gchararray) "en"
// minimum-bitrate: (guint) 198375
// video-codec: (gchararray) "H.264 (High Profile)"
// bitrate: (guint) 2497185
// minimum-bitrate: (guint) 362037
// maximum-bitrate: (guint) 4695459
