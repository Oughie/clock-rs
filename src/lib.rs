mod character;
mod character_display;
mod cli;
mod clock;
mod color;
mod config;
mod position;
mod segment;
mod state;

use std::{env::VarError, fs};

use clap::Parser;

use crate::{cli::Args, config::Config, state::State};

pub fn run() -> Result<(), String> {
    let args = Args::parse();
    let mut config: Config = Config::default();

    if let Some(file_path) = match std::env::var("CONF_PATH") {
        Ok(path) => Ok(Some(path)),
        Err(VarError::NotUnicode(ref s)) => {
            Err(format!("environment variabl is not valid unicode: {:?}", s))
        }
        Err(VarError::NotPresent) => match dirs::config_local_dir() {
            Some(dir) => match dir.join("clock-rs/conf.toml").to_str() {
                Some(path) => Ok(Some(path.to_string())),
                None => unreachable!("configuration path is not valid unicode"),
            },
            None => Ok(None),
        },
    }? {
        let config_str = fs::read_to_string(file_path).map_err(|err| err.to_string())?;
        config = toml::from_str(&config_str).map_err(|err| err.to_string())?;
    }

    args.overwrite(&mut config);

    State::new(&config)
        .map_err(|err| err.to_string())?
        .run()
        .map_err(|err| err.to_string())
}
