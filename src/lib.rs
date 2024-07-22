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
    let file_path = match std::env::var("CONF_PATH") {
        Ok(path) => Ok(path),
        Err(VarError::NotUnicode(ref s)) => Err(format!(
            "environment variable was not valid unicode: {:?}",
            s
        )),
        Err(VarError::NotPresent) => match dirs::config_dir() {
            Some(dir) => match dir.join("clock-rs/conf.toml").to_str() {
                Some(path) => Ok(path.to_string()),
                None => Err("path is not valid unicode".into()),
            },
            None => Err("no config directory found".into()),
        },
    }?;

    let config_str = fs::read_to_string(file_path).map_err(|err| err.to_string())?;

    let mut config: Config = toml::from_str(&config_str).map_err(|err| err.to_string())?;
    let args = Args::parse();
    args.overwrite(&mut config);

    State::new(&config)
        .map_err(|err| err.to_string())?
        .run()
        .map_err(|err| err.to_string())
}
