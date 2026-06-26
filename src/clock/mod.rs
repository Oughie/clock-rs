pub mod counter;
pub mod mode;
pub mod time_zone;

use std::{
    io::{BufWriter, StdoutLock, Write},
    time::Duration,
};

use crate::{
    character::Character,
    clock::mode::ClockMode,
    color::Color,
    config::{Config, TimeFormat},
    error::Error,
    position::Position,
};

#[derive(Default)]
pub struct Padding {
    pub top: u16,
    clock: String,
    text: String,
}

pub struct TimeParts {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}

pub struct Clock {
    pub mode: ClockMode,
    pub padding: Padding,
    pub interval: Duration,
    pub x_pos: Position,
    pub y_pos: Position,
    pub color: Color,
    pub use_12h: bool,
    pub hide_seconds: bool,
    pub clock_format: TimeFormat,
    pub counter_format: TimeFormat,
    pub blink: bool,
    pub bold: bool,
}

impl Clock {
    const DIGIT_WIDTH: u16 = 7;
    const SEPARATOR_WIDTH: u16 = 5;
    const WIDTH: u16 = Self::DIGIT_WIDTH * 6 + Self::SEPARATOR_WIDTH * 2 - 1;
    const WIDTH_NO_SECONDS: u16 = Self::DIGIT_WIDTH * 4 + Self::SEPARATOR_WIDTH - 1;
    const WIDTH_MILLISECONDS: u16 = Self::DIGIT_WIDTH * 9 + Self::SEPARATOR_WIDTH * 3 - 1;
    const HEIGHT: u16 = 7;
    const SUFFIX_LEN: u16 = 5;
    const AM_SUFFIX: &'static str = " [AM]";
    const PM_SUFFIX: &'static str = " [PM]";

    pub fn new(config: Config, mode: ClockMode) -> Self {
        Self {
            mode,
            padding: Padding::default(),
            interval: Duration::from_millis(config.general.interval),
            x_pos: config.position.x,
            y_pos: config.position.y,
            color: config.general.color,
            use_12h: config.date.use_12h,
            hide_seconds: config.date.hide_seconds,
            clock_format: config.clock.fmt,
            counter_format: config.counter.fmt,
            blink: config.general.blink,
            bold: config.general.bold,
        }
    }

    pub fn update_padding(&mut self, width: u16, height: u16) -> Result<(), Error> {
        let clock_width = self.width();
        let text_len = self.mode.text(clock_width)?.len() as u16
            + if self.shows_12h_suffix() {
                Self::SUFFIX_LEN
            } else {
                0
            };

        let half_width = clock_width / 2;

        let column = self.x_pos.calculate(width, half_width);
        self.padding.top = self.y_pos.calculate(height, Self::HEIGHT / 2);

        self.padding.clock = " ".repeat(column as usize);
        self.padding.text = format!(
            "{}{}",
            self.padding.clock,
            " ".repeat(half_width.saturating_sub(text_len / 2) as usize)
        );

        Ok(())
    }

    pub fn is_too_large(&self, width: u16, height: u16) -> bool {
        self.width() + 1 >= width || Self::HEIGHT + 1 >= height
    }

    fn width(&self) -> u16 {
        match self.time_format() {
            format if format.shows_milliseconds() => Self::WIDTH_MILLISECONDS,
            _ if self.hide_seconds => Self::WIDTH_NO_SECONDS,
            _ => Self::WIDTH,
        }
    }

    fn time_format(&self) -> TimeFormat {
        match &self.mode {
            ClockMode::Counter(_) => self.counter_format,
            ClockMode::Time { .. } => self.clock_format,
        }
    }

    fn shows_12h_suffix(&self) -> bool {
        matches!(&self.mode, ClockMode::Time { .. }) && self.use_12h
    }

    fn shows_seconds(&self, time_format: TimeFormat) -> bool {
        !self.hide_seconds || time_format.shows_milliseconds()
    }

    pub fn fmt(&self, w: &mut BufWriter<StdoutLock<'_>>) -> Result<(), Error> {
        let time_format = self.time_format();
        let mut text = self.mode.text(self.width())?;
        let mut time = self.mode.get_time(time_format.shows_milliseconds());

        if self.shows_12h_suffix() {
            let suffix = if time.hour < 12 {
                Self::AM_SUFFIX
            } else {
                Self::PM_SUFFIX
            };

            text.push_str(suffix);

            if time.hour > 12 {
                time.hour -= 12;
            } else if time.hour == 0 {
                time.hour = 12;
            }
        }

        let color = &self.color;

        for row in 0..5 {
            let colon_character = if self.blink && (time.second & 1 == 1) {
                Character::Empty
            } else {
                Character::Colon
            };

            let colon = colon_character.fmt(color, row);
            let h0 = Character::Num(time.hour / 10).fmt(color, row);
            let h1 = Character::Num(time.hour % 10).fmt(color, row);
            let m0 = Character::Num(time.minute / 10).fmt(color, row);
            let m1 = Character::Num(time.minute % 10).fmt(color, row);

            write!(w, "{}{h0}{h1}{colon}{m0}{m1}", self.padding.clock)?;

            if self.shows_seconds(time_format) {
                let s0 = Character::Num(time.second / 10).fmt(color, row);
                let s1 = Character::Num(time.second % 10).fmt(color, row);

                write!(w, "{colon}{s0}{s1}")?;
            }

            if time_format.shows_milliseconds() {
                let dot = Character::Dot.fmt(color, row);
                let ms0 = Character::Num(time.millisecond / 100).fmt(color, row);
                let ms1 = Character::Num(time.millisecond / 10 % 10).fmt(color, row);
                let ms2 = Character::Num(time.millisecond % 10).fmt(color, row);

                write!(w, "{dot}{ms0}{ms1}{ms2}")?;
            }

            writeln!(w, "\r")?;
        }

        let bold_escape_str = if self.bold { Color::BOLD } else { "" };

        writeln!(
            w,
            "\n{bold_escape_str}{}{}{text}",
            self.padding.text,
            self.color.foreground()
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        clock::{
            counter::{Counter, CounterType},
            mode::ClockMode,
        },
        config::{Config, TimeFormat},
    };

    use super::Clock;

    #[test]
    fn whole_second_counter_respects_hide_seconds() {
        let mut config = Config::default();
        config.date.hide_seconds = true;

        let clock = Clock::new(
            config,
            ClockMode::Counter(Counter::new(CounterType::Stopwatch)),
        );

        assert_eq!(clock.width(), Clock::WIDTH_NO_SECONDS);
        assert!(!clock.shows_seconds(clock.time_format()));
    }

    #[test]
    fn millisecond_counter_shows_seconds_when_hide_seconds_is_set() {
        let mut config = Config::default();
        config.date.hide_seconds = true;
        config.counter.fmt = TimeFormat::HoursMinutesSecondsMilliseconds;

        let clock = Clock::new(
            config,
            ClockMode::Counter(Counter::new(CounterType::Stopwatch)),
        );

        assert_eq!(clock.width(), Clock::WIDTH_MILLISECONDS);
        assert!(clock.shows_seconds(clock.time_format()));
    }

    #[test]
    fn millisecond_wall_clock_shows_seconds_when_hide_seconds_is_set() {
        let mut config = Config::default();
        config.date.hide_seconds = true;
        config.clock.fmt = TimeFormat::HoursMinutesSecondsMilliseconds;

        let clock = Clock::new(config, ClockMode::default());

        assert_eq!(clock.width(), Clock::WIDTH_MILLISECONDS);
        assert!(clock.shows_seconds(clock.time_format()));
    }
}
