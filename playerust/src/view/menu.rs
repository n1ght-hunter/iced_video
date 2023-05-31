use iced::widget;

use crate::{
    state::State,
    update::{menu_event::MenuEvent, Message},
    Element,
};

pub fn menu(_state: &State) -> Element {
    widget::container(
        widget::button(widget::text("Open File"))
            .on_press(Message::MenuEvent(MenuEvent::OpenFileDialog)),
    )
    .into()
}
