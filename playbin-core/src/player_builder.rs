//! VideoSettings is used to configure the player before it is created

use std::path::PathBuf;

use iced::futures::{self, SinkExt};

use crate::{BasicPlayer, PlayerMessage};

/// setting when creating a player
#[derive(Clone, Debug)]
pub struct PlayerBuilder {
    /// id of the player used for subscription and accesing player
    pub id: String,
    /// start player in play state
    pub auto_start: bool,
    /// vdieo uri
    pub uri: Option<PathBuf>,
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
    pub fn set_uri(mut self, uri: impl Into<PathBuf>) -> Self {
        self.uri = Some(uri.into());
        self
    }

    /// build a player with the settings
    pub fn build<P: BasicPlayer + std::marker::Send + 'static>(
        self,
    ) -> iced::Subscription<PlayerMessage<P>> {
        iced::Subscription::run_with_id(self.id.clone(), iced::stream::channel( 100,  |mut sender| {
            let settings = self.clone();
            async move {
                println!("creating player");
                let (player, res) = P::create(settings);
                println!("created player");
                let _ = sender.send(PlayerMessage::Player(self.id.clone(), player)).await;
                loop {
                    let message = res.recv().await;
                    match message {
                        Ok(message) => {
                            let _ = sender.send(message).await;
                        }
                        Err(e) => {
                            tracing::error!("error in player: {}", e);
                            futures::pending!()
                        }
                    }
                }
            }
        }))
    }
}
