use chrono::{NaiveDate, Utc};
use tick_cli::EntryList;
use crate::{files, input};

pub fn list_entries() -> std::io::Result<()> {
    let filename = select_date().format("%Y-%m-%d").to_string();
    let entries: EntryList = files::load_entry_list(&filename).expect("Cannot load entries");

    print!("{}", entries);

    Ok(())
}

fn select_date() -> NaiveDate {
    let initial_text = format!("{}", Utc::now().format("%Y-%m-%d"));
    let date = input::date("Select a date", Some(initial_text)).unwrap();

    NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap()
}
