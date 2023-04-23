use iced::{
    widget::{button, slider, container, text},
    Color, application,
};

macro_rules! color {
    ($red:expr, $green:expr, $blue:expr) => {
        Color::from_rgba(
            $red as f32 / 255.0,
            $green as f32 / 255.0,
            $blue as f32 / 255.0,
            0.0,
        )
    };
    ($red:expr, $green:expr, $blue:expr, $opacity:expr) => {
        Color::from_rgba(
            $red as f32 / 255.0,
            $green as f32 / 255.0,
            $blue as f32 / 255.0,
            $opacity as f32 / 255.0,
        )
    };
}

pub struct Theme {
    text: Color,
    svg: Color,

    background: Color,
    currant_line: Color,
    foreground: Color,
    comment: Color,
    cyan: Color,
    green: Color,
    orange: Color,
    pink: Color,
    purple: Color,
    red: Color,
    yellow: Color,

    light_blue: Color,
}

impl Theme {
    pub const NORMAL: Self = Self {
        text: Color::WHITE,
        svg: Color::WHITE,

        background: color!(40, 42, 54),
        currant_line: color!(68, 71, 90),
        foreground: color!(248, 248, 242),
        comment: color!(98, 114, 164),
        cyan: color!(139, 233, 253),
        green: color!(80, 250, 123),
        orange: color!(255, 184, 108),
        pink: color!(255, 121, 198),
        purple: color!(189, 147, 249),
        red: color!(255, 85, 85),
        yellow: color!(241, 250, 140),

        light_blue: color!(46, 144, 255),
    };
}

impl Default for Theme {
    fn default() -> Self {
        Self::NORMAL
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Application {
    #[default]
    Default,
}


impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, style: &Self::Style) -> application::Appearance {
        match style {
            Application::Default => application::Appearance {
                background_color: self.background.into(),
                text_color: self.text,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Button {
    #[default]
    Normal,
    Transparent,
}


impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Button) -> button::Appearance {
        let auto_fill = |background: Color, text: Color| button::Appearance {
            background: background.into(),
            text_color: text,
            border_radius: 2.0,
            ..button::Appearance::default()
        };

        match style {
            Button::Normal => auto_fill(self.light_blue, self.text),
            Button::Transparent => auto_fill(Color::TRANSPARENT, self.text),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        let difference = if &Button::Transparent == style {
            iced::Vector::new(0.0, 0.0)
        } else {
            iced::Vector::new(0.0, 1.0)
        };

        button::Appearance {
            shadow_offset: active.shadow_offset + difference,
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: active.background.map(|background| match background {
                iced::Background::Color(color) => iced::Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

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
                background: self.background.into(),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            Container::Custom(f) => f(self),
        }
    }
}

/*
 * Text
 */
#[derive(Clone, Copy, Default)]
pub enum Text {
    #[default]
    Default,
    Color(Color),
    Custom(fn(&Theme) -> text::Appearance),
}



impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Text::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => Default::default(),
            Text::Color(c) => text::Appearance { color: Some(c) },
            Text::Custom(f) => f(self),
        }
    }
}

/// The style of a slider.
#[derive(Clone, Copy, Default)]
pub enum Slider {
    /// The default style.
    #[default]
    Default,
}

impl slider::StyleSheet for Theme {
    type Style = Slider;

    fn active(&self, style: &Self::Style) -> slider::Appearance {
        match style {
            Slider::Default => {
                let handle = slider::Handle {
                    shape: slider::HandleShape::Circle { radius: 1.0 },
                    color: self.yellow,
                    border_color: self.yellow,
                    border_width: 1.0,
                };

                slider::Appearance {
                    rail_colors: (self.cyan, self.cyan),
                    handle: slider::Handle {
                        color: self.red,
                        border_color: self.red,
                        ..handle
                    },
                }
            }
        }
    }

    fn hovered(&self, style: &Self::Style) -> slider::Appearance {
        match style {
            Slider::Default => {
                let active = self.active(style);

                slider::Appearance {
                    handle: slider::Handle {
                        color: self.green,
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
                        color: self.purple,
                        ..active.handle
                    },
                    ..active
                }
            }
        }
    }
}
