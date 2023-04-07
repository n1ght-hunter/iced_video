use crate::{update::Message, State};

pub fn subscriptions(state: &State) -> iced::Subscription<Message> {
    state.player_handler.subscriptions().map(Message::Video)
}
