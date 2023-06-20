use std::{process, error::Error};
use args::{Command::*, Args};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

mod args;
mod create;

fn main() {
    if let Err(e) = args::get().and_then(try_main) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn try_main(args: Args) -> Result<()> {
    let matched = match args.command {
        List => list(),
        Create => create(),
    }?;

    if matched {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

fn list() -> Result<bool> {
    Ok(true)
}

fn create() -> Result<bool> {
    Ok(create::create_entry().is_ok())
}
