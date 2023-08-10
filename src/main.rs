use std::{process, error::Error};
use args::{Command::*, Args};
use config::Config;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

mod commands;
mod config;
mod args;
mod files;
mod api;
mod ui;
mod cache;
mod http;
mod repository;

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
        Add => add(&config),
        List => list(&config),
        Edit => edit(&config),
        Submit => submit(&config),
        Delete => delete(&config),
    }?;

    if matched {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

fn login(config: &mut Config) -> Result<bool> {
    Ok(commands::auth::login(config).is_ok())
}

fn logout(config: &Config) -> Result<bool> {
    Ok(commands::auth::logout(config).is_ok())
}

fn add(config: &Config) -> Result<bool> {
    commands::auth::check(config);

    Ok(commands::add::add_entry(config).is_ok())
}

fn list(config: &Config) -> Result<bool> {
    commands::auth::check(config);

    Ok(commands::list::list_entries().is_ok())
}

fn edit(config: &Config) -> Result<bool> {
    commands::auth::check(config);

    Ok(commands::edit::edit_entry(config).is_ok())
}

fn submit(config: &Config) -> Result<bool> {
    commands::auth::check(config);

    Ok(commands::submit::submit(config).is_ok())
}

fn delete(config: &Config) -> Result<bool> {
    commands::auth::check(config);

    Ok(commands::delete::delete_entry(config).is_ok())
}

