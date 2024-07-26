mod args;
mod character;
mod character_display;
mod clock;
mod color;
mod config;
mod position;
mod segment;
mod state;

use std::{env::VarError, fs, time::Duration};

use args::Mode;
use clap::Parser;
use clock::{clock_mode::ClockMode, counter::Counter, time::Time};

use crate::{args::Args, config::Config, state::State};

pub fn run() -> Result<(), String> {
    let args = Args::parse();
    let mut config: Config = Config::default();

    if let Some(file_path) = match std::env::var("CONF_PATH") {
        Ok(path) => Ok(Some(path)),
        Err(VarError::NotUnicode(s)) => {
            Err(format!("environment variabl is not valid unicode: {:?}", s))
        }
        Err(VarError::NotPresent) => match dirs::config_local_dir() {
            Some(dir) => match dir.join("clock-rs/conf.toml").to_str() {
                Some(path) => Ok(Some(path.to_string())),
                None => Err("configuration path is not valid unicode".into()),
            },
            None => Ok(None),
        },
    }? {
        let config_str = fs::read_to_string(file_path).map_err(|err| err.to_string())?;
        config = toml::from_str(&config_str).map_err(|err| err.to_string())?;
    }

    let mode = args.mode.clone();
    args.overwrite(&mut config);

    let clock_mode = if let Some(mode) = mode {
        match mode {
            Mode::Clock => ClockMode::CurrentTime(match config.date.utc {
                true => Time::Utc,
                false => Time::Local,
            }),
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
