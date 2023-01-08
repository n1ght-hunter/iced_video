use iced::Command;
use video_player::viewer::PlayerEvent;

use crate::state::State;

use super::Message;

pub fn player_event(state: &mut State, event: PlayerEvent) -> iced::Command<Message> {
    let p = state.player.as_mut();

    if let Some(player) = p {
        match event {
            PlayerEvent::Play => player.set_paused_state(false),
            PlayerEvent::Pause => player.set_paused_state(true),
            PlayerEvent::ToggleMute => {
                if player.muted() {
                    player.set_muted(false)
                } else {
                    player.set_muted(true)
                }
            }
            PlayerEvent::Volume(volume) => player.set_volume(volume),
            PlayerEvent::Seek(p) => {
                state.seek = Some(p as u64);
            }
            PlayerEvent::Released => {
                player.seek(state.seek.unwrap()).unwrap_or_else(|_| ());
                state.seek = None;
            }
        };
    }
    Command::none()
}
