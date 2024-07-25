use chrono::{Local, Timelike, Utc};

pub enum Time {
    Local,
    Utc,
}

impl Time {
    pub fn get_time(&self, date_format: &str) -> (u32, u32, u32, String) {
        match self {
            Self::Utc => {
                let time = Utc::now();
                (
                    time.hour(),
                    time.minute(),
                    time.second(),
                    time.format(date_format).to_string(),
                )
            }
            Self::Local => {
                let time = Local::now();
                (
                    time.hour(),
                    time.minute(),
                    time.second(),
                    time.format(date_format).to_string(),
                )
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
