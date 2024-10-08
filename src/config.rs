use std::{
    env::{self, VarError},
    fs,
    path::Path,
};

use serde::Deserialize;

use crate::{color::Color, error::Error, position::Position};

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub position: PositionConfig,
    pub date: DateConfig,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub color: Color,
    pub interval: u64,
    pub blink: bool,
    pub bold: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            interval: 200,
            color: Color::default(),
            blink: false,
            bold: false,
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct PositionConfig {
    #[serde(rename = "horizontal")]
    pub x: Position,
    #[serde(rename = "vertical")]
    pub y: Position,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct DateConfig {
    pub fmt: String,
    pub use_12h: bool,
    pub utc: bool,
    pub hide_seconds: bool,
}

impl Default for DateConfig {
    fn default() -> Self {
        Self {
            fmt: "%d-%m-%Y".into(),
            use_12h: false,
            utc: false,
            hide_seconds: false,
        }
    }
}

impl Config {
    pub fn parse() -> Result<Self, Error> {
        if let Some(file_path) = match env::var("CONF_PATH") {
            Ok(path) => match path.as_str() {
                "None" => Ok(None),
                _ => Ok(Some(path)),
            },
            Err(VarError::NotUnicode(s)) => {
                Err(Error::PathIsNonUnicode(s.to_string_lossy().to_string()))
            }
            Err(VarError::NotPresent) => match dirs::config_local_dir() {
                Some(dir) => match dir.join("clock-rs/conf.toml").to_str() {
                    Some(path) => match Path::new(path).exists() {
                        true => Ok(Some(path.to_string())),
                        false => Ok(None),
                    },
                    None => Err(Error::PathIsNonUnicode(dir.to_string_lossy().to_string())),
                },
                None => Ok(None),
            },
        }? {
            let config_str = fs::read_to_string(&file_path)
                .map_err(|err| Error::FailedToReadFile(file_path.clone(), err.to_string()))?;
            toml::from_str(&config_str)
                .map_err(|err| Error::InvalidToml(file_path, err.to_string()))
        } else {
            Ok(Config::default())
        }
    }
}
