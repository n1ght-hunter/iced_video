use std::time::Duration;

use iced::Task;
use iced_video::{viewer::ControlEvent, BasicPlayer, AdvancedPlayer};

use crate::state::State;

use super::Message;

pub fn control_event(state: &mut State, event: ControlEvent) -> iced::Task<Message> {
    if let Some(player) = state.player_handler.get_player_mut("main player") {
        match event {
            ControlEvent::Play => player
                .play(),

            ControlEvent::Pause => player
                .pause(),
            ControlEvent::ToggleMute => {
                if player.get_muted() {
                    player.set_muted(false)
                } else {
                    player.set_muted(true)
                }
            }
            ControlEvent::Volume(volume) => player.set_volume(volume),
            ControlEvent::Seek(seek_amount) => {
                state.seek = Some(seek_amount as u64);
            }
            ControlEvent::Released => {
                player
                    .seek(Duration::from_secs(state.seek.unwrap()))
                    .unwrap_or_else(|err| println!("Error seeking: {:?}", err));
                state.seek = None;
            }
        };
    }
    Task::none()
}
