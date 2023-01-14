pub mod menu_event;
pub mod player_event;

use iced::Command;
use video_player::{iced_subscription::SubMSG, viewer::ControlEvent};

use crate::State;

use self::{
    menu_event::{menu_event, MenuEvent},
    player_event::control_event,
};

#[derive(Clone, Debug)]
pub enum Message {
    Video(SubMSG),
    ControlEvent(ControlEvent),
    MenuEvent(MenuEvent),
    SetUri(String),
    None(()),
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
        Message::ControlEvent(event) => return control_event(state, event),
        Message::None(_) => (),
        Message::MenuEvent(event) => return menu_event(state, event),
        Message::SetUri(uri) => {
            if let Some(player) = &state.player {
                player.set_uri(uri);
            }
        }
    }
    Command::none()
}
