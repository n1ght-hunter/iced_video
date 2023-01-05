use iced::{color, widget::slider, Color};

use super::Theme;

/// The style of a slider.
#[derive(Clone, Copy, Default)]
pub enum Slider {
    /// The default style.
    #[default]
    Default,
    Seek,
    Volume,
    /// A custom style.
    Custom(fn(&Theme) -> slider::Appearance),
}

impl slider::StyleSheet for Theme {
    type Style = Slider;

    fn active(&self, style: &Self::Style) -> slider::Appearance {
        match style {
            Slider::Default => slider::Appearance {
                rail_colors: (self.light_blue, self.light_blue),
                handle: slider::Handle {
                    shape: slider::HandleShape::Rectangle {
                        width: 8,
                        border_radius: 4.0,
                    },
                    color: Color::WHITE,
                    border_color: Color::WHITE,
                    border_width: 1.0,
                },
            },
            Slider::Custom(f) => f(self),
            Slider::Seek => slider::Appearance {
                rail_colors: (self.light_blue, color!(143, 143, 143)),
                handle: slider::Handle {
                    shape: slider::HandleShape::Circle { radius: 7.5 },
                    color: color!(150, 150, 150),
                    border_color: color!(97, 97, 97),
                    border_width: 1.0,
                },
            },
            Slider::Volume => slider::Appearance {
                rail_colors: (self.green, color!(143, 143, 143)),
                handle: slider::Handle {
                    shape: slider::HandleShape::Circle { radius: 7.5 },
                    color: color!(150, 150, 150),
                    border_color: color!(97, 97, 97),
                    border_width: 1.0,
                },
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> slider::Appearance {
        match style {
            Slider::Default => {
                let active = self.active(style);

                slider::Appearance {
                    handle: slider::Handle {
                        color: self.orange,
                        ..active.handle
                    },
                    ..active
                }
            }
            Slider::Custom(f) => f(self),
            Slider::Seek => {
                let active = self.active(style);

                slider::Appearance {
                    handle: slider::Handle {
                        color: self.orange,
                        ..active.handle
                    },
                    ..active
                }
            }
            Slider::Volume => {
                let active = self.active(style);

                slider::Appearance {
                    handle: slider::Handle {
                        color: self.orange,
                        ..active.handle
                    },
                    ..active
                }
            }
        }
    }

    fn dragging(&self, style: &Self::Style) -> slider::Appearance {
        match style {
            Slider::Default => {
                let active = self.active(style);

                slider::Appearance {
                    handle: slider::Handle {
                        color: self.orange,
                        ..active.handle
                    },
                    ..active
                }
            }
            Slider::Custom(f) => f(self),
            Slider::Seek => {
                let active = self.active(style);

                slider::Appearance {
                    handle: slider::Handle {
                        color: self.orange,
                        ..active.handle
                    },
                    ..active
                }
            }
            Slider::Volume => {
                let active = self.active(style);

                slider::Appearance {
                    handle: slider::Handle {
                        color: self.orange,
                        ..active.handle
                    },
                    ..active
                }
            }
        }
    }
}
