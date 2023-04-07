use iced::widget;

use crate::{
    state::State,
    theme,
    update::{menu_event::MenuEvent, Message},
    Element,
};

pub fn menu(state: &State) -> Element {
    widget::container(
        widget::button(widget::text("Open File"))
            .on_press(Message::MenuEvent(MenuEvent::OpenFileDialog)),
    )
    .into()
}
