use std::borrow::Cow;

/// setting when creating a player
#[derive(Clone, Debug)]
pub struct VideoSettings {
    /// id of the player used for subscription and accesing player
    pub(crate) id: String,
    /// start player in play state
    pub(crate) auto_start: bool,
    /// if live duration won't work and trying to seek will cause a panic
    pub(crate) live: bool,
    /// vdieo uri
    pub(crate) uri: Option<String>,
}

impl VideoSettings {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            auto_start: false,
            live: false,
            uri: None,
        }
    }

    // start player in play state
    pub fn set_auto_start(mut self, auto_start: bool) -> Self {
        self.auto_start = auto_start;
        self
    }

    // if live duration won't work and trying to seek will cause a panic
    pub fn set_live(mut self, live: bool) -> Self {
        self.live = live;
        self
    }

    // vdieo uri can be set later
    pub fn set_uri(mut self, uri: impl Into<String>) -> Self {
        self.uri = Some(uri.into());
        self
    }
}
