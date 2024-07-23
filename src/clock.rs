use std::{fmt, io};

use chrono::{Local, Timelike, Utc};

use crate::{
    character::Character, character_display::CharacterDisplay, color::Color, config::Config,
    position::Position,
};

#[derive(Default)]
pub struct Clock<'a> {
    x_pos: Position,
    y_pos: Position,
    color: Color,
    date_format: &'a str,
    left_pad: String,
    top_pad: String,
    use_12h: bool,
    date_left_pad: String,
    use_utc: bool,
    hide_seconds: bool,
}

impl<'a> Clock<'a> {
    const WIDTH: usize = 51;
    const WIDTH_NO_SECONDS: usize = 32;
    const HALF_WIDTH: usize = Self::WIDTH / 2;
    const HALF_WIDTH_NO_SECONDS: usize = Self::WIDTH_NO_SECONDS / 2;
    const HEIGHT: usize = 7;
    const HALF_HEIGHT: usize = Self::HEIGHT / 2;
    const SUFFIX_LEN: usize = 5;
    const AM_SUFFIX: &'static str = " [AM]";
    const PM_SUFFIX: &'static str = " [PM]";

    pub fn new(config: &'a Config) -> io::Result<Self> {
        Ok(Self {
            x_pos: config.position.x,
            y_pos: config.position.y,
            color: config.general.color,
            date_format: &config.date.fmt,
            use_12h: config.date.use_12h,
            use_utc: config.date.utc,
            hide_seconds: config.date.hide_seconds,
            ..Default::default()
        })
    }

    pub fn update_position(&mut self, width: u16, height: u16) {
        let date_display = if self.use_utc {
            Utc::now().format(self.date_format)
        } else {
            Local::now().format(self.date_format)
        };

        let date_display_len =
            date_display.to_string().len() + if self.use_12h { Self::SUFFIX_LEN } else { 0 };

        let half_width = if self.hide_seconds {
            Self::HALF_WIDTH_NO_SECONDS
        } else {
            Self::HALF_WIDTH
        };

        let x = self.x_pos.calc(width as usize, half_width);
        let y = self.y_pos.calc(height as usize, Self::HALF_HEIGHT);

        self.left_pad = " ".repeat(x);
        self.top_pad = "\n".repeat(y);
        self.date_left_pad = " ".repeat(x + half_width.saturating_sub(date_display_len / 2));
    }

    pub fn is_too_large(&self, width: usize, height: usize) -> bool {
        if self.hide_seconds {
            if Self::WIDTH_NO_SECONDS + 2 > width {
                return true;
            }
        } else {
            if Self::WIDTH + 2 > width {
                return true;
            }
        }
        if Self::HEIGHT + 2 > height {
            return true;
        }
        false
    }
}

impl fmt::Display for Clock<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hour;
        let minute;
        let mut second = 0;

        let mut date_display = if self.use_utc {
            let utc = Utc::now();
            hour = utc.hour();
            minute = utc.minute();
            if !self.hide_seconds {
                second = utc.second();
            }
            utc.format(self.date_format).to_string()
        } else {
            let local = Local::now();
            hour = local.hour();
            minute = local.minute();
            if !self.hide_seconds {
                second = local.second();
            }
            local.format(self.date_format).to_string()
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

        writeln!(f, "{}", self.top_pad)?;

        let components = if !self.hide_seconds {
            vec![hour, minute, second]
        } else {
            vec![hour, minute]
        };

        for row in 0..5 {
            write!(f, "{}", self.left_pad)?;

            for (i, component) in components.iter().enumerate() {
                let i0 = CharacterDisplay::new(Character::Num(component / 10), self.color, row);
                let i1 = CharacterDisplay::new(Character::Num(component % 10), self.color, row);

                write!(f, "{i0}{i1}")?;

                if i < components.len() - 1 {
                    let colon = CharacterDisplay::new(Character::Colon, self.color, row);
                    write!(f, "{colon}")?;
                }
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
