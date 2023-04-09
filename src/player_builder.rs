//! VideoSettings is used to configure the player before it is created

use crate::{Player, PlayerMessage};

/// setting when creating a player
#[derive(Clone, Debug)]
pub struct PlayerBuilder {
    /// id of the player used for subscription and accesing player
    pub(crate) id: String,
    /// start player in play state
    pub(crate) auto_start: bool,
    /// vdieo uri
    pub(crate) uri: Option<String>,
}

impl PlayerBuilder {
    /// create a new video settings
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            auto_start: false,
            uri: None,
        }
    }

    /// start player in play state
    pub fn set_auto_start(mut self, auto_start: bool) -> Self {
        self.auto_start = auto_start;
        self
    }

    /// vdieo uri can be set later
    pub fn set_uri(mut self, uri: impl Into<String>) -> Self {
        self.uri = Some(uri.into());
        self
    }

    /// build a player with the settings
    pub fn build(self) -> (PlayerMessage, tokio::sync::mpsc::UnboundedReceiver<PlayerMessage>) {
        if cfg!(feature = "gstreamer") {
            Player::new(self)
        } else {
            panic!("No backend selected");
        }
    }
}
