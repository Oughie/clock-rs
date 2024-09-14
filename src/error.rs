use std::io;

use thiserror::Error;

use crate::{clock::counter::Counter, color::Color};

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "The timer duration must be shorter than {} hours:{} {0}s >= {}s",
        Counter::MAX_TIMER_HOURS,
        Color::RESET,
        Counter::MAX_TIMER_SECONDS
    )]
    TimerDurationTooLong(u64),
    #[error(r#"Configuration path is invalid unicode:{} "{0}""#, Color::RESET)]
    PathIsNonUnicode(String),
    #[error(r#"Failed to read file "{0}":{}\n{1}"#, Color::RESET)]
    FailedToReadFile(String, String),
    #[error(r#"Failed to parse configuration file "{0}":{}\n{1}"#, Color::RESET)]
    InvalidToml(String, String),
    #[error("IO Error:{}\n{0}", Color::RESET)]
    Io(#[from] io::Error),
}
