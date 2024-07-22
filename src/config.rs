use serde::Deserialize;

use crate::{color::Color, position::Position};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub position: PositionConfig,
    pub date: DateConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub color: Color,
    pub interval: u64,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            interval: 500,
            color: Color::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct PositionConfig {
    #[serde(rename = "horizontal")]
    pub x: Position,
    #[serde(rename = "vertical")]
    pub y: Position,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DateConfig {
    pub fmt: String,
    pub use_12h: bool,
    pub utc: bool,
}

impl Default for DateConfig {
    fn default() -> Self {
        Self {
            fmt: "%d-%M-%Y".into(),
            use_12h: false,
            utc: false,
        }
    }
}
