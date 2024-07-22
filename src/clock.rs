use std::{fmt, io};

use chrono::{Local, Timelike};

use crate::{
    character::Character, character_display::CharacterDisplay, color::Color, config::Config,
    position::Position,
};

#[derive(Default)]
pub struct Clock<'a> {
    x: u16,
    y: u16,
    x_pos: Position,
    y_pos: Position,
    color: Color,
    left_pad: String,
    top_pad: String,
    date_format: &'a str,
    date_use_12h: bool,
    date_left_pad: String,
}

impl<'a> Clock<'a> {
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
        let date_display_len = date_display.len() as u16 + if self.date_use_12h { 3 } else { 0 };

        self.x = self.x_pos.calc(width, 26);
        self.y = self.y_pos.calc(height, 3);
        self.left_pad = (0..self.x).map(|_| ' ').collect();
        self.top_pad = (0..self.y).map(|_| '\n').collect();
        self.date_left_pad = (0..self.x + 26_u16.saturating_sub(date_display_len / 2))
            .map(|_| ' ')
            .collect();
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
            let suffix = if hour < 12 { " [AM]" } else { " [PM]" };
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
                let i0 = CharacterDisplay::new(self.color, Character::Num(component / 10), row);
                let i1 = CharacterDisplay::new(self.color, Character::Num(component % 10), row);

                write!(f, "{i0}{i1}")?;

                if i < 2 {
                    let c = CharacterDisplay::new(self.color, Character::Colon, row);
                    write!(f, "{c}")?;
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
