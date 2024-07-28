pub mod menu_event;
pub mod player_event;
use std::path::PathBuf;

use iced::Task;
use iced_video::{
    viewer::ControlEvent,
     PlayerMessage,
     BasicPlayer,
};

use crate::{helpers::component_trait::Update, State};

use self::{
    menu_event::{menu_event, MenuEvent},
    player_event::control_event,
};

#[derive(Clone, Debug)]
pub enum Message {
    KeyBoardEvent(iced::keyboard::Event),
    Video(PlayerMessage),
    ControlEvent(ControlEvent),
    MenuEvent(MenuEvent),
    SetUri(String),
    None(()),
}

pub fn update(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::Video(event) => {
            state.player_handler.handle_event(event);
        }
        Message::ControlEvent(event) => return control_event(state, event),
        Message::None(_) => (),
        Message::MenuEvent(event) => return menu_event(state, event),
        Message::SetUri(uri) => {
            if let Some(player) = state.player_handler.get_player_mut("main player") {
                player.set_source(&PathBuf::from(uri));
            }
        }
        Message::KeyBoardEvent(event) => {
            return crate::components::keypress::KeyPressHandler::update(state, event)
        }
    }
    Task::none()
}
