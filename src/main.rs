use std::io;

use crossterm::{cursor::Show, execute, terminal};

fn main() {
    if let Err(err) = clock_rs::run() {
        let mut stdout = io::stdout();

        execute!(stdout, Show).unwrap();
        terminal::disable_raw_mode().unwrap();

        panic!("{err}")
    }
}
