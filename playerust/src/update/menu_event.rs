use std::path::PathBuf;

use iced::Task;
use iced_video::BasicPlayer;
use rfd::AsyncFileDialog;

use crate::{state::State, helpers::open_file::open_file};

use super::Message;

#[derive(Clone, Debug)]
pub enum MenuEvent {
    OpenFileDialog,
    OpenFile(Option<String>),
}

pub fn menu_event(state: &mut State, event: MenuEvent) -> iced::Task<Message> {
    match event {
        MenuEvent::OpenFileDialog => {
            return Task::perform(async { open_file().await }, |f| {
                Message::MenuEvent(MenuEvent::OpenFile(f))
            })
        }
        MenuEvent::OpenFile(file) => {
            if let Some(uri) = file {
                if let Some(player) = state.player_handler.get_player_mut("main player") {
                    player.set_source(&PathBuf::from(uri)).unwrap();
                }
            }
        }
    }
    Task::none()
}
