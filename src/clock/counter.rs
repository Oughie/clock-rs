use std::time::{Duration, Instant};

pub struct Counter {
    start: Instant,
    last_pause: Option<Instant>,
    duration: Option<Duration>,
    paused: bool,
}

impl Counter {
    pub fn new(duration: Option<Duration>) -> Self {
        Self {
            start: Instant::now(),
            last_pause: None,
            duration,
            paused: false,
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.paused {
            if let Some(last_pause) = self.last_pause {
                self.start += last_pause.elapsed();
                self.last_pause = None;
            }
        } else {
            self.last_pause = Some(Instant::now());
        }

        self.paused = !self.paused;
    }

    pub fn restart(&mut self) {
        self.start = Instant::now();
        self.last_pause = None;
        self.paused = false;
    }

    pub fn get_time(&self) -> (u32, u32, u32, String) {
        let elapsed = if self.paused {
            if let Some(last_pause) = self.last_pause {
                last_pause.duration_since(self.start)
            } else {
                Duration::from_secs(0)
            }
        } else {
            self.start.elapsed()
        };

        let elapsed = if let Some(duration) = self.duration {
            duration.saturating_sub(elapsed.saturating_sub(Duration::from_secs(1)))
        } else {
            elapsed
        };

        let secs = elapsed.as_secs() as u32;
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        (hours, minutes, seconds, self.text().into())
    }

    pub const fn text(&self) -> &str {
        if self.paused {
            "p: Toggle Pause, r: Restart [Paused]"
        } else {
            "p: Toggle Pause, r: Restart"
        }
    }
}
