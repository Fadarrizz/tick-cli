use chrono::{NaiveDate, NaiveTime};
use dialoguer::{FuzzySelect, theme::ColorfulTheme, Input, Confirm};

pub fn fuzzy_select<T: ToString>(prompt: &str, items: &[T]) -> Option<usize> {
    Some(FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .interact()
        .unwrap())
}

pub fn date(prompt: &str, initial_text: Option<String>) -> Option<String> {
    Some(Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .with_initial_text(initial_text.unwrap())
        .validate_with(|input: &String| -> Result<(), &str> {
            let date = NaiveDate::parse_from_str(&input, "%Y-%m-%d");
            match date {
                Ok(_date) => Ok(()),
                Err(_e) => Err("Wrong date format. Please provide a date like 2023-01-30")
            }
        })
        .interact()
        .unwrap())
}

pub fn time(prompt: &str) -> Option<String> {
    Some(Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), &str> {
            let time = NaiveTime::parse_from_str(&input, "%H:%M");
            match time {
                Ok(_time) => Ok(()),
                Err(_e) => Err("Wrong time format. Please provide a time like 9:15")
            }
        })
        .interact()
        .unwrap())
}

pub fn default(prompt: &str) -> Option<String> {
    Some(Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()
        .unwrap())
}

pub fn confirm(prompt: &str) -> Option<bool> {
    Some(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(true)
        .wait_for_newline(true)
        .interact()
        .unwrap())
}
