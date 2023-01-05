mod player_event;

use iced::Command;
use video_player::{iced_subscription::SubMSG, viewer::PlayerEvent};

use crate::State;

use self::player_event::player_event;

#[derive(Clone, Debug)]
pub enum Message {
    Video(SubMSG),
    PlayerEvent(PlayerEvent),
    None,
}

pub fn update(state: &mut State, message: Message) -> iced::Command<Message> {
    match message {
        Message::Video(event) => match event {
            SubMSG::Image(_id, image) => {
                state.frame = Some(image);
            }
            SubMSG::Message(_id, message) => {
                println!("message: {:?}", message);
                match message {
                    _ => (),
                }
            }
            SubMSG::Player(_id, player) => state.player = Some(player),
        },
        Message::PlayerEvent(event) => return player_event(state, event),
        Message::None => (),
    }
    Command::none()
}
