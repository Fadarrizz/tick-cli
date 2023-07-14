use std::{process, error::Error};
use args::{Command::*, Args};
use config::Config;
use dialoguer::console::style;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

mod auth;
mod config;
mod args;
mod create;
mod files;
mod api;
mod input;
mod list;

fn main() {
    if let Err(e) = args::get().and_then(try_main) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn try_main(args: Args) -> Result<()> {
    let mut config = config::load()?;

    let matched = match args.command {
        Login => login(&mut config),
        Logout => logout(&config),
        Create => create(&config),
        List => list(&config),
    }?;

    if matched {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

fn login(config: &mut Config) -> Result<bool> {
    Ok(auth::login(config).is_ok())
}

fn logout(config: &Config) -> Result<bool> {
    Ok(auth::logout(config).is_ok())
}

fn create(config: &Config) -> Result<bool> {
    check_auth(config);

    Ok(create::create_entry(config).is_ok())
}

fn list(config: &Config) -> Result<bool> {
    check_auth(config);

    Ok(list::list_entries().is_ok())
}

fn check_auth(config: &Config) {
    if config.missing_api_key() {
        println!("To get started with Tick CLI, please run {}", style("tick login").bold());
        process::exit(1)
    }
}
