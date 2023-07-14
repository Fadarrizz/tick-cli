use std::{process, error::Error};
use args::{Command::*, Args};
use config::Config;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

mod config;
mod args;
mod create;
mod files;
mod api;

fn main() {
    if let Err(e) = args::get().and_then(try_main) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn try_main(args: Args) -> Result<()> {
    let mut config = config::load()?;

    let matched = match args.command {
        Create => create(),
    }?;

    if matched {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

fn create() -> Result<bool> {
    Ok(create::create_entry().is_ok())
}
