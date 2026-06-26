use crate::{
    clock::{counter::Counter, time_zone::TimeZone, TimeParts},
    error::Error,
};

pub enum ClockMode {
    Counter(Counter),
    Time {
        time_zone: TimeZone,
        date_format: String,
    },
}

impl ClockMode {
    pub fn get_time(&self, show_counter_milliseconds: bool) -> TimeParts {
        match self {
            Self::Counter(counter) => counter.get_time(show_counter_milliseconds),
            Self::Time { time_zone, .. } => time_zone.get_time(),
        }
    }

    pub fn text(&self, max_len: u16) -> Result<String, Error> {
        match self {
            Self::Counter(counter) => Ok(counter.text.to_string()),
            Self::Time {
                time_zone,
                date_format,
            } => time_zone.text(date_format, max_len),
        }
    }
}

impl Default for ClockMode {
    fn default() -> Self {
        Self::Time {
            time_zone: TimeZone::Local,
            date_format: "%d-%m-%Y".to_string(),
        }
    }
}
