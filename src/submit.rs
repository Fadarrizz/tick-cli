use std::process;

use chrono::NaiveTime;
use tick_cli::{Entry, TickEntryList};

use crate::{files, input, api, config::Config};

pub fn submit_entries(config: &Config) -> std::io::Result<()> {
    // select file to submit
    let filename = select_file().unwrap();

    // collect entries
    let mut entries = files::load_entry_list(&filename).expect("Cannot load entries");

    // ask to set end time of last entry, if not set
    let last_entry = entries.get_last().unwrap();
    if last_entry.is_missing_end_time() {
        println!("You didn't set an end time for the last entry of this day:");
        println!("{}", last_entry);
        println!("This is a requirement for submitting.");

        set_last_entry_end_time(entries.get_last_mut().unwrap());
    }

    // set end times
    entries.set_end_times();

    // convert entries to tick entries
    let mut tick_entry_list = TickEntryList::from_entry_list(&filename, &entries);

    // merge those that have the same task
    tick_entry_list.merge();

    for entry in tick_entry_list.get_all() {
        match api::send_entry(config, &entry) {
            Ok(()) => continue,
            Err(e) => {
                println!("{}", e.message());
                process::exit(1);
            }
        }
    };

    Ok(())
}

fn select_file() -> Option<String> {
    let existing_files = files::get_existing_file_names();

    match input::fuzzy_select("Select a file", &existing_files, Some(0)) {
        Some(index) => Some(existing_files[index].clone()),
        None => panic!("Nothing selected"),
    }
}

fn set_last_entry_end_time(entry: &mut Entry) {
    let end_time = input_end_time();

    entry.set_end_time(end_time);
}

fn input_end_time() -> NaiveTime {
    input::time("Input end time", None).unwrap()
}
