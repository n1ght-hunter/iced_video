use iced::Command;
use iced_video::PlayerBackend;
use rfd::AsyncFileDialog;

use crate::{state::State, helpers::open_file::open_file};

use super::Message;

#[derive(Clone, Debug)]
pub enum MenuEvent {
    OpenFileDialog,
    OpenFile(Option<String>),
}

pub fn menu_event(state: &mut State, event: MenuEvent) -> iced::Command<Message> {
    match event {
        MenuEvent::OpenFileDialog => {
            return Command::perform(async { open_file().await }, |f| {
                Message::MenuEvent(MenuEvent::OpenFile(f))
            })
        }
        MenuEvent::OpenFile(file) => {
            if let Some(uri) = file {
                if let Some(player) = state.player_handler.get_player_mut("main player") {
                    player.set_source(&uri).unwrap();
                }
            }
        }
    }
    Command::none()
}
