mod character;
mod cli;
mod clock;
mod color;
mod config;
mod error;
mod position;
mod segment;
mod state;

use std::{process, time::Duration};

use clap::Parser;
use clock::time_zone::TimeZone;

use crate::{
    cli::{Args, Mode},
    clock::{counter::Counter, mode::ClockMode},
    config::Config,
    error::Error,
    state::State,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("\x1b[1;4;31mError:\x1B[0;1m {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse();
    let mut config = Config::parse()?;
    let mode = args.mode.clone();
    args.overwrite(&mut config);

    let clock_mode = if let Some(mode) = mode {
        match mode {
            Mode::Clock => ClockMode::Time {
                time_zone: TimeZone::from_bool(config.date.utc),
                date_format: config.date.fmt.clone(),
            },
            Mode::Timer(args) => {
                if args.secs >= Counter::MAX_TIMER_DURATION {
                    return Err(Error::TimerDurationTooLong(args.secs));
                }
                ClockMode::Counter(Counter::new(Some(Duration::from_secs(args.secs))))
            }
            Mode::Stopwatch => ClockMode::Counter(Counter::new(None)),
        }
    } else {
        ClockMode::Time {
            time_zone: TimeZone::from_bool(config.date.utc),
            date_format: config.date.fmt.clone(),
        }
    };

    State::new(config, clock_mode)?.run()?;
    Ok(())
}
