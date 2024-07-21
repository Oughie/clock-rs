use clap::ValueEnum;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    #[default]
    White,
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Reset,
}

impl Color {
    pub const fn foreground(&self) -> &'static str {
        match self {
            Self::White => "\x1B[37m",
            Self::Black => "\x1B[30m",
            Self::Red => "\x1B[31m",
            Self::Green => "\x1B[32m",
            Self::Blue => "\x1B[34m",
            Self::Yellow => "\x1B[33m",
            Self::Magenta => "\x1B[35m",
            Self::Cyan => "\x1B[36m",
            Self::Reset => "\x1B[0m",
        }
    }
    pub const fn background(&self) -> &'static str {
        match self {
            Self::White => "\x1B[47m",
            Self::Black => "\x1B[40m",
            Self::Red => "\x1B[41m",
            Self::Green => "\x1B[42m",
            Self::Blue => "\x1B[44m",
            Self::Yellow => "\x1B[43m",
            Self::Magenta => "\x1B[45m",
            Self::Cyan => "\x1B[46m",
            Self::Reset => "\x1B[0m",
        }
    }
}
