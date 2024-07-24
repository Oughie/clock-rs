use crate::{color::Color, config::Config, position::Position};
use clap::{
    builder::styling::{AnsiColor, Effects, Styles},
    Parser,
};

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::BrightYellow.on_default())
}

#[derive(Parser)]
#[clap(version = "v0.1.0, (C) 2024 Oughie", hide_possible_values = true, styles = styles())]
pub struct Args {
    #[clap(long, short, value_enum)]
    #[clap(help = "Specify the clock color")]
    pub color: Option<Color>,
    #[clap(long, short, value_enum)]
    #[clap(help = "Set the position along the horizontal axis")]
    pub x_pos: Option<Position>,
    #[clap(long, short, value_enum)]
    #[clap(help = "Set the position along the vertical axis")]
    pub y_pos: Option<Position>,
    #[clap(long, value_enum)]
    #[clap(help = "Set the date format")]
    pub fmt: Option<String>,
    #[clap(short = 't')]
    #[clap(help = "Use the 12h format")]
    pub use_12h: bool,
    #[clap(long, short)]
    #[clap(help = "Set the polling interval in milliseconds")]
    pub interval: Option<u64>,
    #[clap(long)]
    #[clap(help = "Use UTC time")]
    pub utc: bool,
    #[clap(long, short = 's')]
    #[clap(help = "Do not show seconds")]
    pub hide_seconds: bool,
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
