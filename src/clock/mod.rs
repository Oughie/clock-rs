pub mod counter;
pub mod mode;
pub mod time_zone;

use std::{fmt, io};

use crate::{character::Character, color::Color, config::Config, position::Position};

use mode::ClockMode;

pub struct Clock {
    pub mode: ClockMode,
    x_pos: Position,
    y_pos: Position,
    color: Color,
    use_12h: bool,
    hide_seconds: bool,
    blink: bool,
    bold: bool,
    left_pad: String,
    top_pad: String,
    text_left_pad: String,
}

impl Clock {
    const WIDTH: usize = 51;
    const WIDTH_NO_SECONDS: usize = 32;
    const HEIGHT: usize = 7;
    const SUFFIX_LEN: usize = 5;
    const AM_SUFFIX: &'static str = " [AM]";
    const PM_SUFFIX: &'static str = " [PM]";

    pub fn new(config: Config, mode: ClockMode) -> io::Result<Self> {
        Ok(Self {
            mode,
            x_pos: config.position.x,
            y_pos: config.position.y,
            color: config.general.color,
            use_12h: config.date.use_12h,
            hide_seconds: config.date.hide_seconds,
            blink: config.general.blink,
            bold: config.general.bold,
            left_pad: String::new(),
            top_pad: String::new(),
            text_left_pad: String::new(),
        })
    }

    pub fn update_position(&mut self, width: u16, height: u16) {
        let text = self.mode.text();

        let text_len = text.to_string().len() + if self.use_12h { Self::SUFFIX_LEN } else { 0 };

        let half_width = if self.hide_seconds {
            Self::WIDTH_NO_SECONDS / 2
        } else {
            Self::WIDTH / 2
        };

        let x = self.x_pos.calculate(width.into(), half_width);
        let y = self.y_pos.calculate(height.into(), Self::HEIGHT / 2);

        self.left_pad = " ".repeat(x);
        self.top_pad = "\n".repeat(y);
        self.text_left_pad = " ".repeat(x + half_width.saturating_sub(text_len / 2));
    }

    pub fn is_too_large(&self, width: usize, height: usize) -> bool {
        if self.hide_seconds {
            if Self::WIDTH_NO_SECONDS + 2 > width {
                return true;
            }
        } else if Self::WIDTH + 2 > width {
            return true;
        }
        Self::HEIGHT + 2 > height
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = self.mode.text();
        let (mut hour, minute, second) = self.mode.get_time();

        match self.mode {
            ClockMode::Time { .. } if self.use_12h => {
                let suffix = if hour < 12 {
                    Self::AM_SUFFIX
                } else {
                    Self::PM_SUFFIX
                };

                text.push_str(suffix);

                if hour > 12 {
                    hour -= 12;
                } else if hour == 0 {
                    hour = 12;
                }
            }
            _ => (),
        }

        let color = &self.color;

        writeln!(f, "{}", self.top_pad)?;

        for row in 0..5 {
            let colon_character = if self.blink {
                if second & 1 == 0 {
                    Character::Colon
                } else {
                    Character::Empty
                }
            } else {
                Character::Colon
            };

            let colon = colon_character.fmt(color, row);
            let h0 = Character::Num(hour / 10).fmt(color, row);
            let h1 = Character::Num(hour % 10).fmt(color, row);
            let m0 = Character::Num(minute / 10).fmt(color, row);
            let m1 = Character::Num(minute % 10).fmt(color, row);

            write!(f, "{}{h0}{h1}{colon}{m0}{m1}", self.left_pad)?;

            if !self.hide_seconds {
                let s0 = Character::Num(second / 10).fmt(color, row);
                let s1 = Character::Num(second % 10).fmt(color, row);
                write!(f, "{colon}{s0}{s1}")?;
            }

            writeln!(f, "\r")?;
        }

        let bold_escape_str = if self.bold { "\x1B[1m" } else { "" };

        writeln!(
            f,
            "\n{bold_escape_str}{}{}{text}",
            self.text_left_pad,
            self.color.foreground(),
        )
    }
}
