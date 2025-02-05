use crate::{color::Color, position::Position};

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
#[clap(version = "v0.1.214, (C) 2024 Oughie", hide_possible_values = true, styles = styles())]
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
