use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, Clear, ClearType},
};

use crate::{clock::Clock, config::Config};

enum Message {
    RedrawRequested,
    Exit,
}
pub struct State<'a> {
    clock: Clock<'a>,
    interval: Duration,
}

impl<'a> State<'a> {
    pub fn new(config: &'a Config) -> io::Result<Self> {
        let (width, height) = terminal::size()?;

        let mut clock = Clock::new(config)?;
        clock.update_position(width, height);

        Ok(Self {
            clock,
            interval: Duration::from_millis(config.general.interval),
        })
    }

    pub fn run(mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(stdout, Clear(ClearType::All), Hide)?;

        self.render()?;

        loop {
            self.render()?;
            if let Some(msg) = self.handle_events()? {
                match msg {
                    Message::RedrawRequested => self.render()?,
                    Message::Exit => break,
                }
            }
        }

        self.close()
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

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

    fn handle_events(&mut self) -> io::Result<Option<Message>> {
        Ok(if event::poll(self.interval)? {
            match event::read()? {
                Event::Key(key_event) => match key_event {
                    KeyEvent {
                        code: KeyCode::Esc | KeyCode::Char('q'),
                        ..
                    } => Some(Message::Exit),
                    _ => None,
                },
                Event::Resize(width, height) => {
                    self.clock.update_position(width, height);
                    Some(Message::RedrawRequested)
                }
                _ => None,
            }
        } else {
            None
        })
    }
}
