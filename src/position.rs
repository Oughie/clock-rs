use clap::ValueEnum;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    #[default]
    Start,
    Center,
    End,
}

impl Position {
    pub fn calc(&self, len: u16, clock_width: u16) -> u16 {
        match self {
            Self::Start => 1,
            Self::Center => (len / 2).saturating_sub(clock_width),
            Self::End => len.saturating_sub(clock_width * 2),
        }
    }
}
