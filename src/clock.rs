use std::{fmt, io};

use chrono::{Local, Timelike};

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
    date_use_12h: bool,
    date_left_pad: String,
}

impl<'a> Clock<'a> {
    const HALF_WIDTH: usize = 26;
    const HALF_HEIGHT: usize = 3;
    const SUFFIX_LEN: usize = 6;
    const AM_SUFFIX: &'static str = " [AM]";
    const PM_SUFFIX: &'static str = " [PM]";

    pub fn new(config: &'a Config) -> io::Result<Self> {
        Ok(Self {
            x_pos: config.position.x,
            y_pos: config.position.y,
            color: config.general.color,
            date_format: &config.date.fmt,
            date_use_12h: config.date.use_12h,
            ..Default::default()
        })
    }

    pub fn update_position(&mut self, width: u16, height: u16) {
        let local = Local::now();
        let date_display = local.format(self.date_format).to_string();
        let date_display_len = date_display.len()
            + if self.date_use_12h {
                Self::SUFFIX_LEN
            } else {
                0
            };

        let x = self.x_pos.calc(width as usize, Self::HALF_WIDTH);
        let y = self.y_pos.calc(height as usize, Self::HALF_HEIGHT);
        self.left_pad = " ".repeat(x);
        self.top_pad = "\n".repeat(y);
        self.date_left_pad = " ".repeat(x + Self::HALF_WIDTH.saturating_sub(date_display_len / 2));
    }
}

impl fmt::Display for Clock<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let local = Local::now();
        let mut hour = local.hour();
        let minute = local.minute();
        let second = local.second();

        let mut date_display = local.format(self.date_format).to_string();

        if self.date_use_12h {
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

        for row in 0..5 {
            write!(f, "{}", self.left_pad)?;

            for (i, component) in [hour, minute, second].iter().enumerate() {
                let i0 = CharacterDisplay::new(Character::Num(component / 10), self.color, row);
                let i1 = CharacterDisplay::new(Character::Num(component % 10), self.color, row);

                write!(f, "{i0}{i1}")?;

                if i < 2 {
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
