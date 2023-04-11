use iced::{color, widget, Alignment, Background, Color, Length, Padding};
use iced_video::{
    helpers::{helper_functions::secs_to_hhmmss, svgs},
    viewer::ControlEvent,
    PlayerBackend,
};

use crate::{state::State, theme, update::Message, Element};

pub fn controls(state: &State) -> Element {
    let player = state.player_handler.get_player("main player");
    let duration = if let Some(p) = player {
        p.get_duration().as_secs()
    } else {
        0
    };
    let position = if let Some(seek) = state.seek {
        seek.to_owned()
    } else if let Some(p) = player {
        p.get_position().as_secs()
    } else {
        0
    };
    let play_pause = if let Some(player) = player {
        if player.get_paused() {
            widget::Button::new(widget::svg(svgs::play_svg()).height(28).width(28))
                .on_press(Message::ControlEvent(ControlEvent::Play))
        } else {
            widget::Button::new(widget::svg(svgs::pause_svg()).height(28).width(28))
                // .style(theme::Button::Transparent)
                .on_press(Message::ControlEvent(ControlEvent::Pause))
        }
    } else {
        widget::Button::new(widget::svg(svgs::play_svg()).height(28).width(28))
            .on_press(Message::ControlEvent(ControlEvent::Play))
    };

    let duration_text = widget::container(widget::text(format!(
        "{} / {}",
        secs_to_hhmmss(position),
        secs_to_hhmmss(duration)
    )))
    .padding([0, 5]);

    let volume = if let Some(player) = player {
        player.get_volume()
    } else {
        1.0
    };
    let volume_svg = if volume > 0.66 {
        svgs::high_volume_svg()
    } else if volume > 0.33 {
        svgs::medium_volume_svg()
    } else if volume > 0.0 {
        svgs::low_volume_svg()
    } else {
        svgs::muted_svg()
    };

    let volume_button = if let Some(player) = player {
        if !player.get_mute() {
            widget::Button::new(widget::svg(volume_svg).height(28).width(28))
                .on_press(Message::ControlEvent(ControlEvent::ToggleMute))
        } else {
            widget::Button::new(widget::svg(svgs::muted_svg()).height(28).width(28))
                .on_press(Message::ControlEvent(ControlEvent::ToggleMute))
        }
    } else {
        widget::Button::new(widget::svg(volume_svg).height(28).width(28))
            .on_press(Message::ControlEvent(ControlEvent::ToggleMute))
    };

    let volume_slider = widget::container(
        widget::Slider::new(0.0..=1.0, volume, |v| {
            Message::ControlEvent(ControlEvent::Volume(v))
        })
        .style(theme::Slider::Volume)
        .step(0.05)
        .width(80),
    )
    .padding([0, 5]);

    let seek_slider = widget::Slider::new(
        0.0..=duration.to_owned() as f64,
        position.to_owned() as f64,
        |v| Message::ControlEvent(ControlEvent::Seek(v)),
    )
    .on_release(Message::ControlEvent(ControlEvent::Released))
    .style(theme::Slider::Seek)
    .step(1.0);

    widget::container(
        widget::column![
            seek_slider,
            widget::row![play_pause, duration_text, volume_button, volume_slider]
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Alignment::Center)
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .style(theme::Container::Custom(|_theme| {
        widget::container::Appearance {
            text_color: None,
            background: Some(Background::Color(color!(242, 241, 236))),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::WHITE,
        }
    }))
    .padding(Padding {
        top: 0.0,
        right: 10.0,
        left: 10.0,
        bottom: 5.0,
    })
    .height(60)
    .width(Length::Fill)
    .into()
}
