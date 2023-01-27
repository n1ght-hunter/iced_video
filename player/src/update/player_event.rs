use iced::Command;
use video_player::viewer::ControlEvent;

use crate::state::State;

use super::Message;

pub fn control_event(state: &mut State, event: ControlEvent) -> iced::Command<Message> {
    let p = state.player.as_mut();

    if let Some(player) = p {
        match event {
            ControlEvent::Play => player.set_playing_state(true),
            ControlEvent::Pause => player.set_playing_state(false),
            ControlEvent::ToggleMute => {
                if player.muted() {
                    player.set_muted(false)
                } else {
                    player.set_muted(true)
                }
            }
            ControlEvent::Volume(volume) => player.set_volume(volume),
            ControlEvent::Seek(p) => {
                state.seek = Some(p as u64);
            }
            ControlEvent::Released => {
                player.seek(state.seek.unwrap()).unwrap_or_else(|_| ());
                state.seek = None;
            }
        };
    }
    Command::none()
}
