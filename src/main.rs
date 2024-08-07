mod args;
mod character;
mod character_display;
mod clock;
mod color;
mod config;
mod position;
mod segment;
mod state;

use std::{process, time::Duration};

use clap::Parser;

use crate::{
    args::{Args, Mode},
    clock::{clock_mode::ClockMode, counter::Counter, time::Time},
    config::Config,
    state::State,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error:\n{err}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse();
    let mut config = Config::parse()?;

    let mode = args.mode.clone();
    args.overwrite(&mut config);

    let clock_mode = if let Some(mode) = mode {
        match mode {
            Mode::Clock => ClockMode::CurrentTime(Time::from_utc(config.date.utc)),
            Mode::Timer(args) => {
                if args.secs >= 360000 {
                    return Err(
                        "the timer duration cannot exceed 359,999 seconds (or 1 day - 1 second)"
                            .into(),
                    );
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

    State::new(config, clock_mode)
        .map_err(|err| err.to_string())?
        .run()
        .map_err(|err| err.to_string())
}
