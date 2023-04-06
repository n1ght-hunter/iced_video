//! A struct that handles all the players and images
//! offers a high level api to interact with the players

use std::collections::HashMap;

use iced::widget::image;

use crate::{
    iced_subscription::{self, PlayerMessage},
    player::VideoPlayer,
    video_settings::VideoSettings,
};

/// A struct that handles all the players and images
#[derive(Debug, Default)]
pub struct PlayerHandler {
    subscriptions: Vec<VideoSettings>,
    players: HashMap<String, VideoPlayer>,
    images: HashMap<String, image::Handle>,
}

impl PlayerHandler {
    /// start a new player
    pub fn start_player(&mut self, settings: VideoSettings) {
        self.subscriptions.push(settings);
    }

    /// the subscriptions for the players
    pub fn subscriptions(&self) -> iced::Subscription<PlayerMessage> {
        let subscriptions = self
            .subscriptions
            .iter()
            .map(|settings| iced_subscription::video_subscription(settings.clone()));
        iced::Subscription::batch(subscriptions)
    }

    /// handle the messages from the subscriptions
    pub fn handle_event(&mut self, message: PlayerMessage) -> Option<(String, gst::Message)> {
        match message {
            PlayerMessage::Player(id, player) => {
                let _ = self.players.insert(id, player);
                None
            }
            PlayerMessage::Image(id, image) => {
                let _ = self.images.insert(id, image);
                None
            }
            PlayerMessage::Message(id, message) => Some((id, message)),
        }
    }
}

impl PlayerHandler {
    /// get a mutable reference to the player
    pub fn get_player_mut(&mut self, id: &str) -> Option<&mut VideoPlayer> {
        self.players.get_mut(id)
    }

    /// get a reference to the player
    pub fn get_player(&self, id: &str) -> Option<&VideoPlayer> {
        self.players.get(id)
    }

    /// get all the players in a hashmap
    pub fn get_all_players(&self) -> &HashMap<String, VideoPlayer> {
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
    pub fn players_and_images(&self) -> Vec<(&String, &VideoPlayer, &image::Handle)> {
        self.players
            .iter()
            .filter_map(|(id, player)| self.images.get(id).map(|image| (id, player, image)))
            .collect()
    }
}
