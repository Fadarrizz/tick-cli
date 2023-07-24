use std::process;

use dialoguer::{Input, theme::ColorfulTheme, Password, console::style};
use tick_cli::Role;

use crate::{api, config::Config};

pub fn login(config: &mut Config) -> std::io::Result<()> {
    let email = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Email")
        .interact()
        .unwrap();

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()
        .unwrap();

    let roles = match api::get_roles(&email, &password) {
        Ok(roles) => roles,
        Err(e) if e.is_unauthenticated_error() => {
            println!("Invalid credentials provided. Please try again.");
            process::exit(1)
        }
        Err(e) => {
            panic!("Unexpected error: {}", e.message());
        }
    };

    let role: &Role = roles.first().unwrap();

    config.set_subscription_id(role.get_subscription_id().clone());
    config.set_api_key(role.get_api_token().clone());

    config.store().expect("Failed logging in");

    let users = match api::get_users(config) {
        Ok(users) => users,
        Err(e) => {
            println!("{}", e.message());
            process::exit(1)
        }
    };
    let first_name = users.first().unwrap().get_first_name();
    println!("Logged in as {} from {}", style(first_name).bold(), style(role.get_company()).bold());

    Ok(())
}

pub fn logout(config: &Config) -> std::io::Result<()> {
    if config.missing_api_key() {
        println!("You are not logged in. Run {} to authenticate.", style("tick login").bold());

        return Ok(());
    }

    match Config::reset() {
        Ok(()) => println!("Logged out successfully"),
        Err(e) => println!("Logging out failed: {}", e),
    }

    Ok(())
}
