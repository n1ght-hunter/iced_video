//! A struct that handles all the players and images
//! offers a high level api to interact with the players

use iced::widget::image;
use playbin_core::{BasicPlayer, PlayerBuilder, PlayerMessage};

use std::collections::HashMap;

/// A struct that handles all the players and images
#[derive(Debug)]
pub struct PlayerHandler<P> {
    subscriptions: Vec<PlayerBuilder>,
    players: HashMap<String, P>,
    images: HashMap<String, image::Handle>,
}

impl<P> Default for PlayerHandler<P> {
    fn default() -> Self {
        Self {
            subscriptions: Vec::new(),
            players: HashMap::new(),
            images: HashMap::new(),
        }
    }
}

impl<P: BasicPlayer + std::marker::Send + 'static> PlayerHandler<P> {
    /// start a new player
    pub fn start_player(&mut self, settings: PlayerBuilder) {
        self.subscriptions.push(settings);
    }

    /// the subscriptions for the players
    pub fn subscriptions(&self) -> iced::Subscription<PlayerMessage<P>> {
        let subscriptions = self
            .subscriptions
            .iter()
            .map(|settings| settings.clone().build());
        iced::Subscription::batch(subscriptions)
    }

    /// handle the messages from the subscriptions
    pub fn handle_event(&mut self, message: PlayerMessage<P>) {
        match message {
            PlayerMessage::Player(id, player) => {
                let _ = self.players.insert(id, player);
            }
            PlayerMessage::Frame(id, image) => {
                let _ = self.images.insert(id, image);
            }
        }
    }
}

impl<P> PlayerHandler<P> {
    /// get a mutable reference to the player
    pub fn get_player_mut(&mut self, id: &str) -> Option<&mut P> {
        self.players.get_mut(id)
    }

    /// get a reference to the player
    pub fn get_player(&self, id: &str) -> Option<&P> {
        self.players.get(id)
    }

    /// get all the players in a hashmap
    pub fn get_all_players(&self) -> &HashMap<String, P> {
        &self.players
    }

    /// get a reference to the image
    pub fn get_frame(&self, id: &str) -> Option<&image::Handle> {
        self.images.get(id)
    }

    /// get all the images in a hashmap
    pub fn get_all_images(&self) -> &HashMap<String, image::Handle> {
        &self.images
    }

    /// get all the players and images zipped together
    /// will only return the players that have an image
    pub fn players_and_images(&self) -> Vec<(&String, &P, &image::Handle)> {
        self.players
            .iter()
            .filter_map(|(id, player)| self.images.get(id).map(|image| (id, player, image)))
            .collect()
    }
}
