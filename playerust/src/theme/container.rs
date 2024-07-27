use iced::{widget::container, Color};

use super::Theme;

/*
 * Container
 */
#[derive(Clone, Copy, Default)]
pub enum Container {
    #[default]
    Transparent,
    Box,
    Custom(fn(&Theme) -> container::Appearance),
}

impl From<fn(&Theme) -> container::Appearance> for Container {
    fn from(f: fn(&Theme) -> container::Appearance) -> Self {
        Self::Custom(f)
    }
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Transparent => Default::default(),
            Container::Box => container::Appearance {
                text_color: None,
                background: Some(self.background.into()),
                border: iced::Border { color: Color::BLACK,  radius: 2.0.into(), ..Default::default() },
                shadow: iced::Shadow::default(),
            },
            Container::Custom(f) => f(self),
        }
    }
}
