use super::{time::Time, time_count::TimeCount};

pub enum ClockMode {
    CurrentTime(Time),
    TimeCount(TimeCount),
}

impl ClockMode {
    pub fn is_time(&self) -> bool {
        matches!(self, Self::CurrentTime(_))
    }

    pub fn get_time(&self, date_format: &str) -> (u32, u32, u32, String) {
        match self {
            Self::CurrentTime(time) => time.get_time(date_format),
            Self::TimeCount(time_count) => time_count.get_time(),
        }
    }

    pub fn text(&self, date_format: &str) -> String {
        match self {
            Self::CurrentTime(time) => time.text(date_format),
            Self::TimeCount(time_count) => time_count.text().into(),
        }
    }
}

impl Default for ClockMode {
    fn default() -> Self {
        Self::CurrentTime(Time::Local)
    }
}
