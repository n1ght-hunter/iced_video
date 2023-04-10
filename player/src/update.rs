pub mod menu_event;
pub mod player_event;

use iced::Command;
use iced_video::{
    gstreamer::{DateTime, MessageView},
    tag_convert::TaglistToTags,
    viewer::ControlEvent,
    PlayerBackend, PlayerMessage,
};

use crate::State;

use self::{
    menu_event::{menu_event, MenuEvent},
    player_event::control_event,
};

#[derive(Clone, Debug)]
pub enum Message {
    Video(PlayerMessage),
    ControlEvent(ControlEvent),
    MenuEvent(MenuEvent),
    SetUri(String),
    None(()),
}

pub fn update(state: &mut State, message: Message) -> iced::Command<Message> {
    match message {
        Message::Video(event) => {
            if let Some((_player_id, message)) = state.player_handler.handle_event(event) {
                match message.view() {
                    MessageView::Tag(tag) => {
                        let tags = tag.tags().to_rust_tags();
                        tags.into_iter().for_each(|(key, value)| {
                            let _ = state.tags.insert(key, value);
                        });
                    }
                    _ => (),
                }
                // println!("message: {:?}", message.view());
            }
        }
        Message::ControlEvent(event) => return control_event(state, event),
        Message::None(_) => (),
        Message::MenuEvent(event) => return menu_event(state, event),
        Message::SetUri(uri) => {
            if let Some(player) = state.player_handler.get_player_mut("main player") {
                player.set_source(&uri).unwrap();
            }
        }
    }
    Command::none()
}
