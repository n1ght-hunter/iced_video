use iced::Command;
use iced_video::viewer::ControlEvent;

use crate::state::State;

use super::Message;

pub fn control_event(state: &mut State, event: ControlEvent) -> iced::Command<Message> {
    if let Some(player) = state.player_handler.get_player_mut("main player".into()) {
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
            ControlEvent::Seek(seek_amount) => {
                state.seek = Some(seek_amount as u64);
            }
            ControlEvent::Released => {
                player
                    .seek(state.seek.unwrap())
                    .unwrap_or_else(|err| println!("Error seeking: {:?}", err));
                state.seek = None;
            }
        };
    }
    Command::none()
}
