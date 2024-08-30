use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{
    clock::{mode::ClockMode, Clock},
    config::Config,
};

pub struct State {
    clock: Clock,
    interval: Duration,
}

impl State {
    pub fn new(config: Config, mode: ClockMode) -> io::Result<Self> {
        let interval = config.general.interval;

        let (width, height) = terminal::size()?;
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
}

impl Drop for State {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        execute!(stdout, LeaveAlternateScreen, Show)
            .expect("Error: Could not leave alternate screen.");
        terminal::disable_raw_mode()
            .expect("Error: Could not disable raw mode. You might have to restart your terminal.");
    }
}
