use std::{
    io::{self, Write},
    time::Duration,
};

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{
    cli::{Args, Mode},
    clock::{counter::Counter, counter::CounterType, mode::ClockMode, time_zone::TimeZone, Clock},
    config::Config,
    error::Error,
};

pub struct State {
    clock: Clock,
    interval: Duration,
}

impl State {
    pub fn new() -> Result<Self, Error> {
        let args = Args::parse();
        let mut config = Config::parse()?;
        let mode = args.mode.clone();
        args.overwrite(&mut config);

        let clock_mode = match mode {
            None | Some(Mode::Clock) => ClockMode::Time {
                time_zone: TimeZone::from_bool(config.date.utc),
                date_format: config.date.fmt.clone(),
            },
            Some(Mode::Timer(args)) => {
                if args.secs >= Counter::MAX_TIMER_SECONDS {
                    return Err(Error::TimerDurationTooLong(args.secs));
                }
                ClockMode::Counter(Counter::new(CounterType::Timer {
                    duration: Duration::from_secs(args.secs),
                    kill: args.kill,
                }))
            }
            Some(Mode::Stopwatch) => ClockMode::Counter(Counter::new(CounterType::Stopwatch)),
        };

        let interval = config.general.interval;

        let (width, height) = terminal::size()?;
        let mut clock = Clock::new(config, clock_mode)?;
        clock.update_position(width, height);

        Ok(Self {
            clock,
            interval: Duration::from_millis(interval),
        })
    }

    pub fn run(mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, Hide)?;

        loop {
            self.render()?;

            if event::poll(self.interval)? {
                match event::read()? {
                    Event::Key(key_event) => match key_event {
                        KeyEvent {
                            code: KeyCode::Esc | KeyCode::Char('q'),
                            modifiers: KeyModifiers::NONE,
                            ..
                        }
                        | KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        } => break Ok(()),
                        KeyEvent {
                            code: KeyCode::Char('p'),
                            kind: KeyEventKind::Press,
                            ..
                        } => {
                            if let ClockMode::Counter(counter) = &mut self.clock.mode {
                                counter.toggle_pause();
                                let (width, height) = terminal::size()?;
                                self.clock.update_position(width, height);
                            }
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                        KeyEvent {
                            code: KeyCode::Char('r'),
                            kind: KeyEventKind::Press,
                            ..
                        } => {
                            if let ClockMode::Counter(counter) = &mut self.clock.mode {
                                counter.restart();
                                let (width, height) = terminal::size()?;
                                self.clock.update_position(width, height);
                            }
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                        _ => (),
                    },
                    Event::Resize(width, height) => {
                        self.clock.update_position(width, height);
                        execute!(stdout, Clear(ClearType::All))?;
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        let (width, height) = terminal::size()?;

        if self.clock.is_too_large(width.into(), height.into()) {
            return Ok(());
        }

        let lock = stdout.lock();
        let mut w = io::BufWriter::new(lock);

        execute!(stdout, MoveTo(0, self.clock.y))?;

        write!(w, "{}", self.clock)?;

        w.flush()
    }

    pub fn exit() {
        let mut stdout = io::stdout();

        execute!(stdout, LeaveAlternateScreen, Show)
            .expect("Error: Could not leave alternate screen.");
        terminal::disable_raw_mode()
            .expect("Error: Could not disable raw mode. You might have to restart your terminal.");
    }
}

impl Drop for State {
    fn drop(&mut self) {
        Self::exit();
    }
}
