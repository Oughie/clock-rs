use crossterm::terminal;

fn main() {
    if let Err(err) = clock_rs::run() {
        terminal::disable_raw_mode().unwrap();
        panic!("{err}")
    }
}
