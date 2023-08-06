use std::{process, error::Error};
use args::{Command::*, Args};
use config::Config;
use dialoguer::console::style;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

mod auth;
mod config;
mod args;
mod add;
mod files;
mod api;
mod input;
mod list;
mod edit;
mod submit;
mod delete;
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
    Ok(auth::login(config).is_ok())
}

fn logout(config: &Config) -> Result<bool> {
    Ok(auth::logout(config).is_ok())
}

fn add(config: &Config) -> Result<bool> {
    check_auth(config);

    Ok(add::add_entry(config).is_ok())
}

fn list(config: &Config) -> Result<bool> {
    check_auth(config);

    Ok(list::list_entries().is_ok())
}

fn edit(config: &Config) -> Result<bool> {
    check_auth(config);

    Ok(edit::edit_entry(config).is_ok())
}

fn submit(config: &Config) -> Result<bool> {
    check_auth(config);

    Ok(submit::submit(config).is_ok())
}

fn delete(config: &Config) -> Result<bool> {
    check_auth(config);

    Ok(delete::delete_entry(config).is_ok())
}

fn check_auth(config: &Config) {
    if config.missing_api_key() {
        println!("To get started with Tick CLI, please run {}", style("tick login").bold());
        process::exit(1)
    }
}
