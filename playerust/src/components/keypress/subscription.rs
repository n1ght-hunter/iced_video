use crate::helpers::component_trait::Subscription;

use super::KeyPressHandler;

impl Subscription for KeyPressHandler {
    type Params = ();

    fn subscription(
        _state: &crate::state::State,
        _params: Self::Params,
    ) -> iced::Subscription<crate::update::Message> {
        iced::event::listen_with(|event, _, _| {
            if let iced::Event::Keyboard(key_event) = event {
                Some(crate::update::Message::KeyBoardEvent(key_event))
            } else {
                None
            }
        })
    }
}
