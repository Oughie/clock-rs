use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};

use crate::{
    clock::{clock_mode::ClockMode, Clock},
    config::Config,
};

pub struct State {
    clock: Clock,
    interval: Duration,
}

impl State {
    pub fn new(config: Config, mode: ClockMode) -> io::Result<Self> {
        let (width, height) = terminal::size()?;

        let interval = config.general.interval;
        let mut clock = Clock::new(config, mode)?;
        clock.update_position(width, height);

        Ok(Self {
            clock,
            interval: Duration::from_millis(interval),
        })
    }

    pub fn run(mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(stdout, Clear(ClearType::All), Hide)?;

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
                        } => break,
                        KeyEvent {
                            code: KeyCode::Char('p'),
                            kind: KeyEventKind::Press,
                            ..
                        } => {
                            if let ClockMode::TimeCount(time_count) = &mut self.clock.mode {
                                time_count.toggle_pause();
                                let (width, height) = terminal::size()?;
                                self.clock.update_position(width, height);
                            }
                        }
                        KeyEvent {
                            code: KeyCode::Char('r'),
                            kind: KeyEventKind::Press,
                            ..
                        } => {
                            if let ClockMode::TimeCount(time_count) = &mut self.clock.mode {
                                time_count.restart();
                            }
                        }
                        _ => (),
                    },
                    Event::Resize(width, height) => {
                        self.clock.update_position(width, height);
                    }
                    _ => (),
                }
            }
        }

        self.close()
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

        let (width, height) = terminal::size()?;
        if self.clock.is_too_large(width.into(), height.into()) {
            return Ok(());
        }

        let lock = stdout.lock();
        let mut w = io::BufWriter::new(lock);

        write!(w, "{}", self.clock)?;

        w.flush()
    }

    fn close(self) -> io::Result<()> {
        let mut stdout = io::stdout();

        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All), Show)?;
        terminal::disable_raw_mode()
    }
}
