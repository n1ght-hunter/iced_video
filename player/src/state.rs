use iced::{Command};
use iced_video::{player_handler::PlayerHandler};

use crate::update::Message;

pub struct State {
    pub player_handler: PlayerHandler,
    pub seek: Option<u64>,
    pub title: String,
}

impl State {
    pub fn new() -> (State, Command<Message>) {
        (
            State {
                player_handler: PlayerHandler::default(),
                seek: None,
                title: String::from("Video Player"),
            },
            Command::none(),
        )
    }
}
