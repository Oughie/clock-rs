use crate::{color::Color, config::Config, position::Position};

use clap::{
    builder::styling::{AnsiColor, Effects, Styles},
    Parser, Subcommand,
};
use serde::Deserialize;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Yellow.on_default() | Effects::ITALIC)
}

#[derive(Parser)]
#[clap(version = "v0.1.210, (C) 2024 Oughie", hide_possible_values = true, styles = styles())]
pub struct Args {
    #[clap(subcommand)]
    pub mode: Option<Mode>,
    #[doc = "Specify the clock color"]
    #[clap(long, short)]
    pub color: Option<Color>,
    #[doc = "Set the position along the horizontal axis"]
    #[clap(long, short)]
    pub x_pos: Option<Position>,
    #[doc = "Set the position along the vertical axis"]
    #[clap(long, short)]
    pub y_pos: Option<Position>,
    #[doc = "Set the date format"]
    #[clap(long)]
    pub fmt: Option<String>,
    #[doc = "Use the 12h format"]
    #[clap(short = 't')]
    pub use_12h: bool,
    #[doc = "Set the polling interval in milliseconds"]
    #[clap(long, short)]
    pub interval: Option<u64>,
    #[doc = "Use UTC time"]
    #[clap(long)]
    pub utc: bool,
    #[doc = "Do not show seconds"]
    #[clap(long, short = 's')]
    pub hide_seconds: bool,
    #[doc = "Set the colon to blink"]
    #[clap(long, short = 'B')]
    pub blink: bool,
    #[doc = "Use bold text"]
    #[clap(long, short)]
    pub bold: bool,
}

#[derive(Clone, Subcommand, Deserialize)]
pub enum Mode {
    #[doc = "Display the current time (default)"]
    Clock,
    #[doc = "Create a timer"]
    Timer(TimerArgs),
    #[doc = "Start a stopwatch"]
    Stopwatch,
}

#[derive(clap::Args, Clone, Deserialize)]
pub struct TimerArgs {
    #[doc = "Specify the timer duration in seconds"]
    pub secs: u64,
    #[doc = "Terminate the application when the timer finishes"]
    #[clap(long, short)]
    pub kill: bool,
}

impl Args {
    pub fn overwrite(self, config: &mut Config) {
        if let Some(color) = self.color {
            config.general.color = color;
        }
        if let Some(interval) = self.interval {
            config.general.interval = interval;
        }
        if let Some(x_pos) = self.x_pos {
            config.position.x = x_pos;
        }
        if let Some(y_pos) = self.y_pos {
            config.position.y = y_pos;
        }
        if self.blink {
            config.general.blink = true;
        }
        if self.bold {
            config.general.bold = true;
        }
        if let Some(fmt) = self.fmt {
            config.date.fmt = fmt;
        }
        if self.use_12h {
            config.date.use_12h = true;
        }
        if self.utc {
            config.date.utc = true;
        }
        if self.hide_seconds {
            config.date.hide_seconds = true;
        }
    }
}
