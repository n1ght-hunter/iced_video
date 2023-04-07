use iced::Command;
use iced_video::{player_handler::PlayerHandler, video_settings::VideoSettings};

use crate::update::Message;

pub struct State {
    pub player_handler: PlayerHandler,
    pub seek: Option<u64>,
    pub title: String,
}

impl State {
    pub fn new() -> (State, Command<Message>) {
        let mut player_handler = PlayerHandler::default();

        player_handler.start_player(VideoSettings::new("main player").set_auto_start(true));
        (
            State {
                player_handler,
                seek: None,
                title: String::from("Video Player"),
            },
            Command::none(),
        )
    }
}
