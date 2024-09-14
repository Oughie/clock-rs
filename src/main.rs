mod character;
mod cli;
mod clock;
#[macro_use]
mod color;
mod config;
mod error;
mod position;
mod segment;
mod state;

use std::process;

use crate::{error::Error, state::State};

fn main() {
    if let Err(err) = (|| -> Result<(), Error> {
        State::new()?.run()?;
        Ok(())
    })() {
        println!("{}Error:{} {err}", esc!("1;3;31"), esc!("0;1"));
        process::exit(1);
    }
}
