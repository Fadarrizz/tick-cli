use crate::{api, config::Config, http::HttpError, repository, ui, files};
use chrono::NaiveTime;
use std::{process, path::PathBuf};
use tick_cli::{Entry, EntryList, TickEntry, TickEntryList};

pub fn submit(config: &Config) -> std::io::Result<()> {
    let path = ui::select_file();
    let mut entries = repository::load_entry_list(&path).expect("Cannot load entries");

    if entries.all_submitted() {
        println!("Everything up-to-date");
        return Ok(());
    }

    set_entry_end_times(&mut entries);

    if confirm_submit().is_none() {
        return Ok(());
    }

    submit_entries(
        config,
        &path,
        &TickEntryList::from_entry_list(
            &files::get_filename_from_path(&path).unwrap(),
            &entries,
        ),
    );

    Ok(())
}

fn set_entry_end_times(entries: &mut EntryList) {
    let last_entry = entries.get_last().unwrap();
    if last_entry.is_missing_end_time() {
        println!("You didn't set an end time for the last entry of this day:");
        println!("{}", last_entry);
        println!("This is a requirement for submitting.");

        set_last_entry_end_time(entries.get_last_mut().unwrap());
    }

    entries.set_end_times();
}

fn set_last_entry_end_time(entry: &mut Entry) {
    let end_time = input_end_time();

    entry.set_end_time(end_time);
}

fn input_end_time() -> NaiveTime {
    ui::time("Input end time", None, false).unwrap()
}

fn confirm_submit() -> Option<bool> {
    ui::confirm("Are you sure you want to submit these entries?")
}

fn submit_entries(config: &Config, path: &PathBuf, tick_entries: &TickEntryList) {
    let mut entries = EntryList::empty();
    let mut errors = Vec::new();
    let mut submitted_count = 0;

    for tick_entry in tick_entries.get_all() {
        let mut entry = tick_entry.get_entry().unwrap().to_owned();

        if entry.is_submitted() && !entry.should_be_updated() {
            entries.add(entry);
            continue;
        }

        let response: Result<TickEntry, HttpError>;
        if entry.should_be_updated() {
            response = api::update_entry(config, &tick_entry);
        } else {
            response = api::create_entry(config, &tick_entry);
        }

        match response {
            Ok(res_tick_entry) => {
                entry.set_tick_id(res_tick_entry.get_id().unwrap());
                entry.set_submitted_at();
                submitted_count += 1;
            }
            Err(e) => errors.push((tick_entry.get_entry().unwrap(), e.message().clone())),
        };

        entries.add(entry);
    }

    if errors.is_empty() {
        entries.set_all_submitted(true);
    }

    repository::store_entry_list(&entries, &path).expect("Unable to store entry list");

    if !errors.is_empty() {
        for (entry, message) in errors {
            println!(
                "Couldn't send the following entry:\n {}\nError: {}",
                entry, message
            );
        }
        process::exit(1);
    }

    println!("Submitted {} entries", submitted_count);
}
