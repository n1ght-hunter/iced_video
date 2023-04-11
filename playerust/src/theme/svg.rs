use iced::widget::svg;

use super::Theme;

/**
 * Svg
 */
#[derive(Default)]
pub enum Svg {
    /// No filtering to the rendered SVG.
    #[default]
    Default,
    Control,
    /// A custom style.
    Custom(fn(&Theme) -> svg::Appearance),
}

impl svg::StyleSheet for Theme {
    type Style = Svg;

    fn appearance(&self, style: &Self::Style) -> svg::Appearance {
        match style {
            Svg::Default => svg::Appearance {
                color: Some(self.svg),
            },
            Svg::Control => svg::Appearance {
                color: Some(self.svg),
            },
            Svg::Custom(f) => f(self),
        }
    }
}
