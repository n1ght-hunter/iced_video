mod controls;
mod image;

use iced::{
    widget::{self},
    Length,
};

use crate::{Element, State};

pub fn view(state: &State) -> Element {
    widget::container(widget::column![
        image::image(state),
        controls::controls(state)
    ])
    .into()
}
