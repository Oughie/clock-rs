use clap::ValueEnum;
use serde::Deserialize;

#[derive(Clone, Default, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Start,
    #[default]
    Center,
    End,
}

impl Position {
    pub fn calc(&self, len: usize, offset: usize) -> usize {
        match self {
            Self::Start => 1,
            Self::Center => (len / 2).saturating_sub(offset),
            Self::End => len.saturating_sub(offset * 2),
        }
    }
}
