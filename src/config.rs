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
    pub clock: ClockConfig,
    pub counter: CounterConfig,
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
            fmt: "%d-%m-%Y".to_string(),
            use_12h: false,
            utc: false,
            hide_seconds: false,
        }
    }
}

pub type ClockConfig = TimeFormatConfig;
pub type CounterConfig = TimeFormatConfig;

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct TimeFormatConfig {
    pub fmt: TimeFormat,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(try_from = "String")]
pub enum TimeFormat {
    #[default]
    HoursMinutesSeconds,
    HoursMinutesSecondsMilliseconds,
}

impl TimeFormat {
    pub const HOURS_MINUTES_SECONDS: &'static str = "hh:mm:ss";
    pub const HOURS_MINUTES_SECONDS_MILLISECONDS: &'static str = "hh:mm:ss.SSS";

    pub fn shows_milliseconds(self) -> bool {
        matches!(self, Self::HoursMinutesSecondsMilliseconds)
    }
}

impl TryFrom<String> for TimeFormat {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for TimeFormat {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            Self::HOURS_MINUTES_SECONDS => Ok(Self::HoursMinutesSeconds),
            Self::HOURS_MINUTES_SECONDS_MILLISECONDS => Ok(Self::HoursMinutesSecondsMilliseconds),
            _ => Err(format!(
                "expected `{}` or `{}`",
                Self::HOURS_MINUTES_SECONDS,
                Self::HOURS_MINUTES_SECONDS_MILLISECONDS
            )),
        }
    }
}

impl Config {
    pub fn parse() -> Result<Self, Error> {
        let path = match env::var("CONF_PATH") {
            Ok(path) => match path.as_str() {
                "None" => None,
                _ => Some(path),
            },
            Err(VarError::NotUnicode(path)) => {
                return Err(Error::NonUnicodePath(path.display().to_string()));
            }
            Err(VarError::NotPresent) => match dirs::config_local_dir() {
                Some(config_local_dir) => {
                    match config_local_dir.join("clock-rs").join("conf.toml").to_str() {
                        Some(path) if Path::new(path).exists() => Some(path.to_string()),
                        Some(_) => None,
                        None => {
                            return Err(Error::NonUnicodePath(
                                config_local_dir.display().to_string(),
                            ))
                        }
                    }
                }
                None => None,
            },
        };

        let Some(file_path) = path else {
            return Ok(Config::default());
        };

        let config_str = fs::read_to_string(&file_path).map_err(|err| Error::ReadFile {
            path: file_path.clone(),
            err: err.to_string(),
        })?;

        toml::from_str(&config_str).map_err(|err| Error::ParseToml {
            path: file_path,
            err: err.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, TimeFormat};

    #[test]
    fn time_formats_default_to_whole_seconds() {
        let config: Config = toml::from_str("").unwrap();

        assert_eq!(config.clock.fmt, TimeFormat::HoursMinutesSeconds);
        assert_eq!(config.counter.fmt, TimeFormat::HoursMinutesSeconds);
    }

    #[test]
    fn clock_format_supports_milliseconds() {
        let config: Config = toml::from_str(
            r#"
            [clock]
            fmt = "hh:mm:ss.SSS"
            "#,
        )
        .unwrap();

        assert_eq!(
            config.clock.fmt,
            TimeFormat::HoursMinutesSecondsMilliseconds
        );
    }

    #[test]
    fn counter_format_supports_milliseconds() {
        let config: Config = toml::from_str(
            r#"
            [counter]
            fmt = "hh:mm:ss.SSS"
            "#,
        )
        .unwrap();

        assert_eq!(
            config.counter.fmt,
            TimeFormat::HoursMinutesSecondsMilliseconds
        );
    }

    #[test]
    fn counter_format_rejects_unknown_format() {
        let err = match toml::from_str::<Config>(
            r#"
            [counter]
            fmt = "HH:MM:SS.SSS"
            "#,
        ) {
            Ok(_) => panic!("invalid counter format was accepted"),
            Err(err) => err,
        };

        assert!(err.to_string().contains("expected `hh:mm:ss`"));
    }
}
