use std::{
    process,
    time::{Duration, Instant},
};

use crate::{clock::TimeParts, state::State};

pub struct Counter {
    pub text: &'static str,
    ty: CounterType,
    start: Instant,
    last_pause: Option<Instant>,
    paused: bool,
}

pub enum CounterType {
    Stopwatch,
    Timer { duration: Duration, kill: bool },
}

impl Counter {
    pub const DEFAULT_TIMER_DURATION: u64 = 5 * 60;
    pub const MAX_TIMER_DURATION: u64 = 99 * 3600 + 59 * 60 + 59;
    const TEXT: &'static str = "P: Toggle Pause, R: Restart";
    const TEXT_PAUSED: &'static str = "P: Toggle Pause, R: Restart [Paused]";

    pub fn new(ty: CounterType) -> Self {
        Self {
            text: Self::TEXT,
            ty,
            start: Instant::now(),
            last_pause: None,
            paused: false,
        }
    }

    pub fn toggle_pause(&mut self) {
        self.text = if self.paused {
            if let Some(last_pause) = self.last_pause {
                self.start += last_pause.elapsed();
                self.last_pause = None;
            }
            Self::TEXT
        } else {
            self.last_pause = Some(Instant::now());
            Self::TEXT_PAUSED
        };

        self.paused = !self.paused;
    }

    pub fn restart(&mut self) {
        self.start = Instant::now();
        self.last_pause = None;

        if self.paused {
            self.toggle_pause();
        }
    }

    pub fn exit_if_finished(&self) {
        if self.should_exit() {
            State::exit();
            process::exit(0);
        }
    }

    fn elapsed(&self) -> Duration {
        if self.paused {
            match self.last_pause {
                Some(last_pause) => last_pause.duration_since(self.start),
                _ => Duration::from_secs(0),
            }
        } else {
            self.start.elapsed()
        }
    }

    fn should_exit(&self) -> bool {
        let elapsed = self.elapsed();

        matches!(
            self.ty,
            CounterType::Timer {
                duration,
                kill: true
            } if elapsed >= duration
        )
    }

    pub fn get_time(&self, show_milliseconds: bool) -> TimeParts {
        let mut elapsed = self.elapsed();
        let mut secs = elapsed.as_secs() as u32;

        if let CounterType::Timer { duration, .. } = self.ty {
            elapsed = if show_milliseconds {
                duration.saturating_sub(elapsed)
            } else {
                duration.saturating_sub(elapsed.saturating_sub(Duration::from_secs(1)))
            };
            secs = elapsed.as_secs() as u32;
        }

        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        TimeParts {
            hour: hours,
            minute: minutes,
            second: seconds,
            millisecond: elapsed.subsec_millis(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{Counter, CounterType};

    fn paused_timer(duration: Duration, elapsed: Duration, kill: bool) -> Counter {
        let start = Instant::now();

        Counter {
            text: Counter::TEXT_PAUSED,
            ty: CounterType::Timer { duration, kill },
            start,
            last_pause: Some(start + elapsed),
            paused: true,
        }
    }

    #[test]
    fn timer_whole_seconds_keeps_previous_countdown_rounding() {
        let counter = paused_timer(Duration::from_secs(5), Duration::from_millis(4200), false);
        let time = counter.get_time(false);

        assert_eq!(time.second, 1);
        assert_eq!(time.millisecond, 800);
    }

    #[test]
    fn timer_milliseconds_use_exact_remaining_time() {
        let counter = paused_timer(Duration::from_secs(5), Duration::from_millis(4200), false);
        let time = counter.get_time(true);

        assert_eq!(time.second, 0);
        assert_eq!(time.millisecond, 800);
    }

    #[test]
    fn timer_kill_uses_elapsed_deadline() {
        let counter = paused_timer(Duration::from_secs(5), Duration::from_secs(5), true);

        assert!(counter.should_exit());
    }

    #[test]
    fn paused_timer_before_deadline_does_not_exit() {
        let counter = paused_timer(Duration::from_secs(5), Duration::from_secs(4), true);

        assert!(!counter.should_exit());
    }
}
