use chrono::{NaiveDate, NaiveTime};
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input};

pub fn fuzzy_select<T: ToString>(
    prompt: &str,
    items: &[T],
    default: Option<usize>,
    opt: bool,
) -> Option<usize> {
    let theme = &ColorfulTheme::default();
    let mut select = FuzzySelect::with_theme(theme);

    // if opt == true {
    //     println!("Skip with <esc>");
    // }

    select
        .with_prompt(prompt)
        .items(items);

    if default.is_some() {
        select.default(default.unwrap());
    }

    if opt == true {
        select.interact_opt().unwrap()
    } else {
        Some(select.interact().unwrap())
    }
}

pub fn date(prompt: &str, default: Option<&String>) -> Option<NaiveDate> {
    let theme = &ColorfulTheme::default();
    let mut input = Input::with_theme(theme);

    input
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), &str> {
            let date = NaiveDate::parse_from_str(&input, "%Y-%m-%d");
            match date {
                Ok(_date) => Ok(()),
                Err(_e) => Err("Wrong date format. Please provide a date like 2023-01-30"),
            }
        });

    // Use inital text, instead of default so that the user
    // can change the input easier.
    if default.is_some() {
        input.with_initial_text(default.unwrap());
    }

    let date = input.interact().unwrap();

    Some(NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap())
}

pub fn time(prompt: &str, default: Option<&String>, opt: bool) -> Option<NaiveTime> {
    let theme = &ColorfulTheme::default();
    let mut input = Input::with_theme(theme);

    input
        .allow_empty(opt)
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), &str> {
            if opt == true && input.is_empty() {
                return Ok(());
            }

            let time = NaiveTime::parse_from_str(&input, "%H:%M");
            match time {
                Ok(_) => Ok(()),
                Err(_) => Err("Wrong time format. Please provide a time like 9:15"),
            }
        });

    // Use inital text, instead of default so that the user
    // can change the input easier.
    if default.is_some() {
        input.with_initial_text(default.unwrap());
    }

    let time = input.interact().unwrap();

    if time.is_empty() {
        None
    } else {
        Some(NaiveTime::parse_from_str(&time, "%H:%M").unwrap())
    }
}

pub fn default(prompt: &str, default: Option<&String>) -> String {
    let theme = &ColorfulTheme::default();
    let mut input = Input::with_theme(theme);

    input
        .allow_empty(true)
        .with_prompt(prompt);

    if default.is_some() {
        input.with_initial_text(default.unwrap());
    }

    input.interact().unwrap()
}

pub fn confirm(prompt: &str) -> Option<bool> {
    Some(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(true)
        .wait_for_newline(true)
        .interact()
        .unwrap())
}
