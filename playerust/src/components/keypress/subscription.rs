use crate::helpers::component_trait::Subscription;

use super::KeyPressHandler;

impl Subscription for KeyPressHandler {
    type Params = ();

    fn subscription(
        state: &crate::state::State,
        params: Self::Params,
    ) -> iced::Subscription<crate::update::Message> {
        iced::subscription::events_with(|event, _| {
            if let iced::Event::Keyboard(key_event) = event {
                Some(crate::update::Message::KeyBoardEvent(key_event))
            } else {
                None
            }
        })
    }
}
