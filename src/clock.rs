use std::{fmt, io};

use chrono::{Local, Timelike, Utc};

use crate::{
    character::Character, character_display::CharacterDisplay, color::Color, config::Config,
    position::Position,
};

#[derive(Default)]
pub struct Clock {
    x_pos: Position,
    y_pos: Position,
    color: Color,
    date_format: String,
    left_pad: String,
    top_pad: String,
    use_12h: bool,
    date_left_pad: String,
    use_utc: bool,
    hide_seconds: bool,
}

impl Clock {
    const WIDTH: usize = 51;
    const WIDTH_NO_SECONDS: usize = 32;
    const HEIGHT: usize = 7;
    const SUFFIX_LEN: usize = 5;
    const AM_SUFFIX: &'static str = " [AM]";
    const PM_SUFFIX: &'static str = " [PM]";

    pub fn new(config: Config) -> io::Result<Self> {
        Ok(Self {
            x_pos: config.position.x,
            y_pos: config.position.y,
            color: config.general.color,
            date_format: config.date.fmt,
            use_12h: config.date.use_12h,
            use_utc: config.date.utc,
            hide_seconds: config.date.hide_seconds,
            ..Default::default()
        })
    }

    pub fn update_position(&mut self, width: u16, height: u16) {
        let date_display = if self.use_utc {
            Utc::now().format(&self.date_format)
        } else {
            Local::now().format(&self.date_format)
        };

        let date_display_len =
            date_display.to_string().len() + if self.use_12h { Self::SUFFIX_LEN } else { 0 };

        let half_width = if self.hide_seconds {
            Self::WIDTH_NO_SECONDS / 2
        } else {
            Self::WIDTH / 2
        };

        let x = self.x_pos.calc(width.into(), half_width);
        let y = self.y_pos.calc(height.into(), Self::HEIGHT / 2);

        self.left_pad = " ".repeat(x);
        self.top_pad = "\n".repeat(y);
        self.date_left_pad = " ".repeat(x + half_width.saturating_sub(date_display_len / 2));
    }

    pub fn is_too_large(&self, width: usize, height: usize) -> bool {
        if self.hide_seconds {
            if Self::WIDTH_NO_SECONDS + 2 > width {
                return true;
            }
        } else if Self::WIDTH + 2 > width {
            return true;
        }
        if Self::HEIGHT + 2 > height {
            return true;
        }
        false
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (mut hour, minute, second, mut date_display) = if self.use_utc {
            let time = Utc::now();
            (
                time.hour(),
                time.minute(),
                if self.hide_seconds { 0 } else { time.second() },
                time.format(&self.date_format).to_string(),
            )
        } else {
            let time = Local::now();
            (
                time.hour(),
                time.minute(),
                if self.hide_seconds { 0 } else { time.second() },
                time.format(&self.date_format).to_string(),
            )
        };

        if self.use_12h {
            let suffix = if hour < 12 {
                Self::AM_SUFFIX
            } else {
                Self::PM_SUFFIX
            };

            date_display.push_str(suffix);

            if hour > 12 {
                hour -= 12;
            } else if hour == 0 {
                hour = 12;
            }
        }

        let color = self.color;

        writeln!(f, "{}", self.top_pad)?;

        for row in 0..5 {
            let colon = CharacterDisplay::new(Character::Colon, color, row);
            let h0 = CharacterDisplay::new(Character::Num(hour / 10), color, row);
            let h1 = CharacterDisplay::new(Character::Num(hour % 10), color, row);
            let m0 = CharacterDisplay::new(Character::Num(minute / 10), color, row);
            let m1 = CharacterDisplay::new(Character::Num(minute % 10), color, row);

            write!(f, "{}{h0}{h1}{colon}{m0}{m1}", self.left_pad)?;

            if !self.hide_seconds {
                let s0 = CharacterDisplay::new(Character::Num(second / 10), color, row);
                let s1 = CharacterDisplay::new(Character::Num(second % 10), color, row);
                write!(f, "{colon}{s0}{s1}")?;
            }

            writeln!(f, "\r")?;
        }

        writeln!(
            f,
            "\n{}{}{date_display}\x1B[0m",
            self.date_left_pad,
            self.color.foreground(),
        )
    }
}
