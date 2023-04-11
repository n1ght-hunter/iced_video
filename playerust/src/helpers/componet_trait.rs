use iced::Command;

use crate::{Element, Message, State};

pub trait Subscription {
    type Params: std::fmt::Debug;

    fn subscription(state: &State, params: Self::Params) -> iced::Subscription<Message>;
}

pub trait View {
    fn view(state: &State) -> Element;
}

pub trait Update {
    type Message: std::fmt::Debug + Send;

    fn update(state: &mut State, params: Self::Message) -> Command<Message>;
}
