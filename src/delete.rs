use tick_cli::{EntryList, Entry};

use crate::{config::Config, files, input, api};

pub fn delete_entry(config: &Config) -> std::io::Result<()> {
    let filename = select_file().unwrap();
    let entries: EntryList = files::load_entry_list(&filename).expect("Cannot load entries");

    let (index, entry) = select_entry(&entries).unwrap();

    if confirm_deletion().is_none() {
        return Ok(())
    }

    if let Some(id) = entry.get_tick_id() {
        api::delete_entry(config, *id).expect("Unable to delete entry from tickspot");
    }

    let mut updated_entries = entries.clone();

    updated_entries.remove(index);

    if updated_entries.is_empty() {
        files::delete_file(&filename).expect("Unable to delete file");
    } else {
        files::store_entry_list(updated_entries, &filename).expect("Unable to store entry list");
    }

    println!("Entry succesfully removed");

    Ok(())
}

fn select_file() -> Option<String> {
    let existing_files = files::get_existing_file_names();

    match input::fuzzy_select("Select a date", &existing_files, Some(0), false) {
        Some(index) => Some(existing_files[index].clone()),
        None => panic!("Nothing selected"),
    }
}

fn select_entry(entry_list: &EntryList) -> Option<(usize, &Entry)> {
    let entries = entry_list.get_all();

    match input::fuzzy_select("Select an entry", entries, Some(0), false) {
        Some(index) => Some((index.clone(), entry_list.get(index))),
        None => panic!("Nothing selected"),
    }
}

fn confirm_deletion() -> Option<bool> {
    input::confirm("Are you sure you want to delete this entry?")
}
