mod controls;
mod image;
mod menu;

use iced::widget;

use crate::{Element, State};

pub fn view(state: &State) -> Element {
    widget::container(widget::column![
        menu::menu(state),
        image::image(state),
        controls::controls(state)
    ])
    .into()
}
