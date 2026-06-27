use std::{
    io::{self, BufWriter, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

#[cfg(unix)]
use signal_hook::{consts, flag};

use crate::{
    cli::args::{Args, Mode, TimerArgs},
    clock::{
        counter::{Counter, CounterType},
        mode::ClockMode,
        time_zone::TimeZone,
        Clock,
    },
    config::Config,
    error::Error,
};

pub struct State {
    clock: Clock,
}

impl State {
    pub fn new() -> Result<Self, Error> {
        let args = Args::parse();
        let mut config = Config::parse()?;
        let mode = args.mode.clone();

        args.overwrite(&mut config)?;

        let clock_mode = Self::clock_mode(mode, &config)?;
        let mut clock = Clock::new(config, clock_mode);

        let (width, height) = terminal::size().map_err(Error::Io)?;
        clock.update_padding(width, height)?;

        Ok(Self { clock })
    }

    fn clock_mode(mode: Option<Mode>, config: &Config) -> Result<ClockMode, Error> {
        let TimerArgs {
            seconds,
            minutes,
            hours,
            kill,
        } = match mode {
            Some(Mode::Clock) | None => {
                return Ok(ClockMode::Time {
                    time_zone: TimeZone::from_utc(config.date.utc),
                    date_format: config.date.fmt.clone(),
                });
            }
            Some(Mode::Stopwatch) => {
                return Ok(ClockMode::Counter(Counter::new(CounterType::Stopwatch)))
            }
            Some(Mode::Timer(timer_args)) => timer_args,
        };

        let total_seconds = match (seconds, minutes, hours) {
            (None, None, None) => Counter::DEFAULT_TIMER_DURATION,
            _ => {
                let seconds = seconds.unwrap_or_default();
                let minutes = minutes.unwrap_or_default();
                let hours = hours.unwrap_or_default();
                let total_seconds = hours * 3600 + minutes * 60 + seconds;

                if total_seconds > Counter::MAX_TIMER_DURATION {
                    return Err(Error::TimerDurationTooLong {
                        hours,
                        minutes,
                        seconds,
                    });
                }

                total_seconds
            }
        };

        Ok(ClockMode::Counter(Counter::new(CounterType::Timer {
            duration: Duration::from_secs(total_seconds),
            kill,
        })))
    }

    pub fn run(mut self) -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;

        let reload_config = Arc::new(AtomicBool::new(false));

        #[cfg(unix)]
        flag::register(consts::SIGUSR1, Arc::clone(&reload_config))?;

        loop {
            if reload_config.swap(false, Ordering::Relaxed) {
                self.reload_config()?;
            }

            self.exit_if_finished();
            self.render()?;

            if !event::poll(self.clock.interval)? {
                continue;
            }

            match event::read()? {
                Event::Key(key_event) => match key_event {
                    KeyEvent {
                        code: KeyCode::Esc | KeyCode::Char('Q' | 'q'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    }
                    | KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => return Ok(()),
                    KeyEvent {
                        code: KeyCode::Char('r'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => reload_config.store(true, Ordering::Relaxed),
                    KeyEvent {
                        code: KeyCode::Char(character @ ('P' | 'p' | 'R' | 'r')),
                        kind: KeyEventKind::Press,
                        modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                        ..
                    } => {
                        let ClockMode::Counter(counter) = &mut self.clock.mode else {
                            continue;
                        };

                        match character {
                            'P' | 'p' => counter.toggle_pause(),
                            _ => counter.restart(),
                        }

                        let (width, height) = terminal::size()?;
                        self.refresh_display(width, height)?;
                    }
                    _ => (),
                },
                Event::Resize(width, height) => self.refresh_display(width, height)?,

                _ => (),
            }
        }
    }

    pub fn exit() {
        execute!(io::stdout(), LeaveAlternateScreen, Show).expect(
            "error: failed to leave alternate screen, you might have to restart your terminal",
        );
        terminal::disable_raw_mode()
            .expect("error: failed to disable raw mode, you might have to restart your terminal");
    }

    fn refresh_display(&mut self, width: u16, height: u16) -> Result<(), Error> {
        execute!(io::stdout(), Clear(ClearType::All))?;
        self.clock.update_padding(width, height)
    }

    fn reload_config(&mut self) -> Result<(), Error> {
        let clock = &mut self.clock;
        let config = Config::parse()?;

        clock.color = config.general.color;
        clock.interval = Duration::from_millis(config.general.interval);
        clock.blink = config.general.blink;
        clock.bold = config.general.bold;

        clock.x_pos = config.position.x;
        clock.y_pos = config.position.y;

        clock.use_12h = config.date.use_12h;
        clock.hide_seconds = config.date.hide_seconds;
        clock.clock_format = config.clock.fmt;
        clock.counter_format = config.counter.fmt;

        if let ClockMode::Time {
            time_zone,
            date_format,
        } = &mut self.clock.mode
        {
            *time_zone = TimeZone::from_utc(config.date.utc);
            *date_format = config.date.fmt;
        }

        let (width, height) = terminal::size()?;
        self.refresh_display(width, height)
    }

    fn exit_if_finished(&self) {
        if let ClockMode::Counter(counter) = &self.clock.mode {
            counter.exit_if_finished();
        }
    }

    fn render(&self) -> Result<(), Error> {
        let (width, height) = terminal::size()?;

        if self.clock.is_too_large(width, height) {
            return Ok(());
        }

        let mut stdout = io::stdout();

        execute!(stdout, MoveTo(0, self.clock.padding.top))?;

        let lock = stdout.lock();
        let mut buffered_writer = BufWriter::new(lock);

        self.clock.fmt(&mut buffered_writer)?;
        buffered_writer.flush()?;

        Ok(())
    }
}

impl Drop for State {
    fn drop(&mut self) {
        Self::exit();
    }
}
