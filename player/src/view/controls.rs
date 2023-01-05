use iced::{color, widget, Alignment, Background, Color, Length, Padding};
use video_player::{svgs, viewer::PlayerEvent};

use crate::{state::State, theme, update::Message, Element};

pub fn controls(state: &State) -> Element {
    let duration = if let Some(p) = &state.player {
        p.duration().as_secs()
    } else {
        0
    };
    let position = if let Some(seek) = state.seek {
        seek.to_owned()
    } else if let Some(p) = &state.player {
        p.position().as_secs()
    } else {
        0
    };
    let play_pause = if let Some(player) = &state.player {
        if player.paused() {
            widget::Button::new(
                widget::svg(svgs::play_svg())
                    .height(Length::Units(28))
                    .width(Length::Units(28)),
            )
            .on_press(Message::PlayerEvent(PlayerEvent::Play))
        } else {
            widget::Button::new(
                widget::svg(svgs::pause_svg())
                    .height(Length::Units(28))
                    .width(Length::Units(28)),
            )
            // .style(theme::Button::Transparent)
            .on_press(Message::PlayerEvent(PlayerEvent::Pause))
        }
    } else {
        widget::Button::new(
            widget::svg(svgs::play_svg())
                .height(Length::Units(28))
                .width(Length::Units(28)),
        )
        .on_press(Message::PlayerEvent(PlayerEvent::Play))
    };

    let duration_text = widget::container(widget::text(format!(
        "{} / {}",
        to_hhmmss(position),
        to_hhmmss(duration)
    )))
    .padding([0, 5]);

    let volume = if let Some(player) = &state.player {
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

    let volume_button = if let Some(player) = &state.player {
        if !player.muted() {
            widget::Button::new(
                widget::svg(volume_svg)
                    .height(Length::Units(28))
                    .width(Length::Units(28)),
            )
            .on_press(Message::PlayerEvent(PlayerEvent::ToggleMute))
        } else {
            widget::Button::new(
                widget::svg(svgs::muted_svg())
                    .height(Length::Units(28))
                    .width(Length::Units(28)),
            )
            .on_press(Message::PlayerEvent(PlayerEvent::ToggleMute))
        }
    } else {
        widget::Button::new(
            widget::svg(volume_svg)
                .height(Length::Units(28))
                .width(Length::Units(28)),
        )
        .on_press(Message::PlayerEvent(PlayerEvent::ToggleMute))
    };

    let volume_slider = widget::container(
        widget::Slider::new(0.0..=1.0, volume, |v| {
            Message::PlayerEvent(PlayerEvent::Volume(v))
        })
        .style(theme::Slider::Volume)
        .step(0.05)
        .width(Length::Units(80)),
    )
    .padding([0, 5]);

    let seek_slider = widget::Slider::new(
        0.0..=duration.to_owned() as f64,
        position.to_owned() as f64,
        |v| Message::PlayerEvent(PlayerEvent::Seek(v)),
    )
    .on_release(Message::PlayerEvent(PlayerEvent::Released))
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
        top: 0,
        right: 10,
        left: 10,
        bottom: 5,
    })
    .height(Length::Units(60))
    .width(Length::Fill)
    .into()
}

fn to_hhmmss(seconds: u64) -> String {
    let (hours, seconds) = (seconds / 3600, seconds % 3600);
    let (minutes, seconds) = (seconds / 60, seconds % 60);
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else if minutes > 1 {
        format!("{}:{:02}", minutes, seconds)
    } else {
        format!("0:{:02}", seconds)
    }
}
