use clap::ValueEnum;
use serde::Deserialize;

#[derive(Clone, Default, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    #[default]
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub const RESET: &'static str = "\x1B[0m";

    pub const fn foreground(&self) -> &'static str {
        match self {
            Self::Black => "\x1B[30m",
            Self::Red => "\x1B[31m",
            Self::Green => "\x1B[32m",
            Self::Yellow => "\x1B[33m",
            Self::Blue => "\x1B[34m",
            Self::Magenta => "\x1B[35m",
            Self::Cyan => "\x1B[36m",
            Self::White => "\x1B[37m",
            Self::BrightBlack => "\x1B[90m",
            Self::BrightRed => "\x1B[91m",
            Self::BrightGreen => "\x1B[92m",
            Self::BrightYellow => "\x1B[93m",
            Self::BrightBlue => "\x1B[94m",
            Self::BrightMagenta => "\x1B[95m",
            Self::BrightCyan => "\x1B[96m",
            Self::BrightWhite => "\x1B[97m",
        }
    }
    pub const fn background(&self) -> &'static str {
        match self {
            Self::Black => "\x1B[40m",
            Self::Red => "\x1B[41m",
            Self::Green => "\x1B[42m",
            Self::Yellow => "\x1B[43m",
            Self::Blue => "\x1B[44m",
            Self::Magenta => "\x1B[45m",
            Self::Cyan => "\x1B[46m",
            Self::White => "\x1B[47m",
            Self::BrightBlack => "\x1B[100m",
            Self::BrightRed => "\x1B[101m",
            Self::BrightGreen => "\x1B[102m",
            Self::BrightYellow => "\x1B[103m",
            Self::BrightBlue => "\x1B[104m",
            Self::BrightMagenta => "\x1B[105m",
            Self::BrightCyan => "\x1B[106m",
            Self::BrightWhite => "\x1B[107m",
        }
    }
}
