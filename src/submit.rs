use std::process;

use chrono::NaiveTime;
use tick_cli::{Entry, TickEntryList, EntryList};

use crate::{files, input, api, config::Config};

pub fn submit_entries(config: &Config) -> std::io::Result<()> {
    let filename = select_file().unwrap();

    let mut entries = files::load_entry_list(&filename).expect("Cannot load entries");

    let last_entry = entries.get_last().unwrap();
    if last_entry.is_missing_end_time() {
        println!("You didn't set an end time for the last entry of this day:");
        println!("{}", last_entry);
        println!("This is a requirement for submitting.");

        set_last_entry_end_time(entries.get_last_mut().unwrap());
    }

    entries.set_end_times();

    let tick_entry_list = TickEntryList::from_entry_list(&filename, &entries);

    entries = EntryList::empty();
    let mut errors = Vec::new();
    for tick_entry in tick_entry_list.get_all() {
        let mut entry = tick_entry.get_entry().unwrap().to_owned();

        match api::send_entry(config, &tick_entry) {
            Ok(res_tick_entry) => entry.set_tick_id(res_tick_entry.get_id().unwrap()),
            Err(e) => errors.push((tick_entry.get_entry().unwrap(), e.message().clone()))
        };

        entries.add(entry);
    }

    files::store_entry_list(entries, &filename).expect("Unable to store entry list");

    if !&errors.is_empty() {
        for (entry, message) in errors {
            println!("Couldn't send the following entry:\n {}\nError: {}", entry, message);
        }
        process::exit(1);
    } else {
        println!("Succesfully sent all entries.");
    }

    Ok(())
}

fn select_file() -> Option<String> {
    let existing_files = files::get_existing_file_names();

    match input::fuzzy_select("Select a file", &existing_files, Some(0), false) {
        Some(index) => Some(existing_files[index].clone()),
        None => panic!("Nothing selected"),
    }
}

fn set_last_entry_end_time(entry: &mut Entry) {
    let end_time = input_end_time();

    entry.set_end_time(end_time);
}

fn input_end_time() -> NaiveTime {
    input::time("Input end time", None, false).unwrap()
}
