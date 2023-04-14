use crate::{
    components::keypress::KeyPressHandler, helpers::component_trait::Subscription, update::Message,
    State,
};

pub fn subscriptions(state: &State) -> iced::Subscription<Message> {
    iced::Subscription::batch(vec![
        KeyPressHandler::subscription(state, ()),
        state.player_handler.subscriptions().map(Message::Video),
    ])
}
