use std::error::Error;
use clap::{Parser,Subcommand};

type ArgResult<T> = Result<T, Box<dyn Error>>;

#[derive(Subcommand, Debug)]
pub enum Command {
    Login,
    Logout,
    Add,
    List,
    Edit,
    Submit,
}

#[derive(Parser, Debug)]
#[command(bin_name = "tick")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

pub fn get() -> ArgResult<Args> {
    Ok(Args::parse())
}
