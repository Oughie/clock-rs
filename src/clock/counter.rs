use std::time::{Duration, Instant};

pub struct Counter {
    pub text: &'static str,
    start: Instant,
    last_pause: Option<Instant>,
    ty: CounterType,
    paused: bool,
}

pub enum CounterType {
    Stopwatch,
    Timer { duration: Duration },
}

impl Counter {
    pub const MAX_TIMER_HOURS: u64 = 24;
    pub const MAX_TIMER_DURATION: u64 = Self::MAX_TIMER_HOURS * 3600;
    const TEXT: &'static str = "p: Toggle Pause, r: Restart";
    const TEXT_PAUSED: &'static str = "p: Toggle Pause, r: Restart [Paused]";

    pub fn new(ty: CounterType) -> Self {
        Self {
            start: Instant::now(),
            last_pause: None,
            ty,
            text: Self::TEXT,
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
        self.text = if self.paused {
            Self::TEXT_PAUSED
        } else {
            Self::TEXT
        };
    }

    pub fn restart(&mut self) {
        self.start = Instant::now();
        self.last_pause = None;
        self.paused = false;
    }

    pub fn get_time(&self) -> (u32, u32, u32) {
        let mut elapsed = if self.paused {
            if let Some(last_pause) = self.last_pause {
                last_pause.duration_since(self.start)
            } else {
                Duration::from_secs(0)
            }
        } else {
            self.start.elapsed()
        };

        if let CounterType::Timer { duration, .. } = self.ty {
            elapsed = duration.saturating_sub(elapsed.saturating_sub(Duration::from_secs(1)))
        }

        let secs = elapsed.as_secs() as u32;
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        (hours, minutes, seconds)
    }
}
