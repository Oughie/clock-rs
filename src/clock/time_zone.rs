use chrono::{Local, Timelike, Utc};

pub enum TimeZone {
    Local,
    Utc,
}

impl TimeZone {
    pub fn from_bool(utc: bool) -> Self {
        match utc {
            true => Self::Utc,
            false => Self::Local,
        }
    }

    pub fn get_time(&self) -> (u32, u32, u32) {
        match self {
            Self::Local => {
                let time = Local::now();
                (time.hour(), time.minute(), time.second())
            }
            Self::Utc => {
                let time = Utc::now();
                (time.hour(), time.minute(), time.second())
            }
        }
    }

    pub fn text(&self, date_format: &str) -> String {
        match self {
            Self::Local => Local::now().format(date_format),
            Self::Utc => Utc::now().format(date_format),
        }
        .to_string()
    }
}
