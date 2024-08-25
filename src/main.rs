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

use crate::{
    cli::{Args, Mode},
    clock::{clock_mode::ClockMode, counter::Counter, time::Time},
    config::Config,
    error::Error,
    state::State,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("\x1b[1;31mError:\x1B[0;1m {err}");
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
            Mode::Clock => ClockMode::CurrentTime(Time::new(config.date.utc)),
            Mode::Timer(args) => {
                if args.secs >= Counter::MAX_TIMER_DURATION {
                    return Err(Error::TimerDurationTooLong(args.secs));
                }
                ClockMode::Counter(Counter::new(Some(Duration::from_secs(args.secs))))
            }
            Mode::Stopwatch => ClockMode::Counter(Counter::new(None)),
        }
    } else {
        ClockMode::CurrentTime(match config.date.utc {
            true => Time::Utc,
            false => Time::Local,
        })
    };

    State::new(config, clock_mode)?.run()?;
    Ok(())
}
