use iced::{widget::image, Command};
use video_player::player::VideoPlayer;

use crate::update::Message;

pub struct State {
    pub player: Option<VideoPlayer>,
    pub uri: Option<String>,
    pub frame: Option<image::Handle>,
    pub seek: Option<u64>,
    pub title: String,
}

impl State {
    pub fn new() -> (State, Command<Message>) {
        (
            State {
                player: None,
                frame: None,
                seek: None,
                uri: None,
                title: String::from("Video Player"),
            },
            Command::none(),
        )
    }
}
