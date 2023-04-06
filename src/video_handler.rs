use std::collections::HashMap;

use iced::widget::image;

use crate::{
    iced_subscription::{self, PlayerMessage},
    player::VideoPlayer,
    video_settings::VideoSettings,
};

#[derive(Debug, Default)]
pub struct PlayerHandler {
    subscriptions: Vec<VideoSettings>,
    players: HashMap<String, VideoPlayer>,
    images: HashMap<String, image::Handle>,
}

impl PlayerHandler {
    pub fn start_player(&mut self, settings: VideoSettings) {
        self.subscriptions.push(settings);
    }

    pub fn subscriptions(&self) -> iced::Subscription<PlayerMessage> {
        let subscriptions = self
            .subscriptions
            .iter()
            .map(|settings| iced_subscription::video_subscription(settings.clone()));
        iced::Subscription::batch(subscriptions)
    }

    pub fn handle_event(&mut self, message: PlayerMessage) -> Option<(String, gst::Message)> {
        match message {
            PlayerMessage::Player(id, player) => {
                self.players.insert(id, player);
                None
            }
            PlayerMessage::Image(id, image) => {
                self.images.insert(id, image);
                None
            }
            PlayerMessage::Message(id, message) => Some((id, message)),
        }
    }
}

impl PlayerHandler {
    pub fn get_player_mut(&mut self, id: &str) -> Option<&mut VideoPlayer> {
        self.players.get_mut(id)
    }

    pub fn get_player(&self, id: &str) -> Option<&VideoPlayer> {
        self.players.get(id)
    }

    pub fn get_all_players(&self) -> &HashMap<String, VideoPlayer> {
        &self.players
    }

    pub fn get_frame(&self, id: &str) -> Option<&image::Handle> {
        self.images.get(id)
    }

    pub fn get_all_images(&self) -> &HashMap<String, image::Handle> {
        &self.images
    }

    pub fn players_and_images(&self) -> Vec<(&String,&VideoPlayer, &image::Handle)> {
        self.players
            .iter()
            .filter_map(|(id, player)| {
                self.images
                    .get(id)
                    .map(|image| (id,player, image))
            })
            .collect()
    }
}
