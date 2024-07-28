
use iced::{color, Color};


pub struct Theme {
    pub text: Color,
    pub svg: Color,

    pub background: Color,
    pub currant_line: Color,
    pub foreground: Color,
    pub comment: Color,
    pub cyan: Color,
    pub green: Color,
    pub orange: Color,
    pub pink: Color,
    pub purple: Color,
    pub red: Color,
    pub yellow: Color,

    pub light_blue: Color,
}

impl Theme {
    pub const NORMAL: Self = Self {
        text: Color::BLACK,
        svg: Color::BLACK,

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
