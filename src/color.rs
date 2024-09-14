use clap::ValueEnum;
use serde::Deserialize;

#[macro_export]
macro_rules! esc {
    ($n:tt) => {
        concat!("\x1B[", $n, "m")
    };
}

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
    pub const RESET: &'static str = esc!(0);
    pub const BOLD: &'static str = esc!(1);

    pub const fn foreground(&self) -> &'static str {
        match self {
            Self::Black => esc!(30),
            Self::Red => esc!(31),
            Self::Green => esc!(32),
            Self::Yellow => esc!(33),
            Self::Blue => esc!(34),
            Self::Magenta => esc!(35),
            Self::Cyan => esc!(36),
            Self::White => esc!(37),
            Self::BrightBlack => esc!(90),
            Self::BrightRed => esc!(91),
            Self::BrightGreen => esc!(92),
            Self::BrightYellow => esc!(93),
            Self::BrightBlue => esc!(94),
            Self::BrightMagenta => esc!(95),
            Self::BrightCyan => esc!(96),
            Self::BrightWhite => esc!(97),
        }
    }
    pub const fn background(&self) -> &'static str {
        match self {
            Self::Black => esc!(40),
            Self::Red => esc!(41),
            Self::Green => esc!(42),
            Self::Yellow => esc!(43),
            Self::Blue => esc!(44),
            Self::Magenta => esc!(45),
            Self::Cyan => esc!(46),
            Self::White => esc!(47),
            Self::BrightBlack => esc!(100),
            Self::BrightRed => esc!(101),
            Self::BrightGreen => esc!(102),
            Self::BrightYellow => esc!(103),
            Self::BrightBlue => esc!(104),
            Self::BrightMagenta => esc!(105),
            Self::BrightCyan => esc!(106),
            Self::BrightWhite => esc!(107),
        }
    }
}
