use iced::{widget, Background, Color, Length};
use iced_video::{viewer::ControlEvent, PlayerBackend};

use crate::{state::State, theme, update::Message, Element};

pub fn image(state: &State) -> Element {
    let image = if let Some(handle) = state.player_handler.get_frame("main player") {
        iced::widget::image(handle.clone())
            .height(Length::Fill)
            .width(Length::Fill)
    } else {
        iced::widget::image(widget::image::Handle::from_pixels(0, 0, vec![]))
            .height(Length::Fill)
            .width(Length::Fill)
    };

    widget::container(
        widget::button(image)
            .on_press(
                if let Some(player) = state.player_handler.get_player("main player") {
                    if player.get_paused() {
                        Message::ControlEvent(ControlEvent::Play)
                    } else {
                        Message::ControlEvent(ControlEvent::Pause)
                    }
                } else {
                    Message::None(())
                },
            )
            .style(theme::Button::Transparent),
    )
    .height(Length::Fill)
    .width(Length::Fill)
    .style(theme::Container::Custom(|_theme| {
        widget::container::Appearance {
            text_color: None,
            background: Some(Background::Color(Color::BLACK)),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::WHITE,
        }
    }))
    .into()
}
