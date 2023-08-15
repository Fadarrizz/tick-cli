use tick_cli::{EntryList, Entry};
use crate::{config::Config, api, repository, ui};

pub fn delete_entry(config: &Config) -> std::io::Result<()> {
    let filename = ui::select_file();
    let entries = repository::load_entry_list(&filename).expect("Cannot load entries");

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
        repository::delete_entry_list(&filename).expect("Unable to delete file");
    } else {
        repository::store_entry_list(updated_entries, &filename).expect("Unable to store entry list");
    }

    println!("Entry succesfully removed");

    Ok(())
}

fn select_entry(entry_list: &EntryList) -> Option<(usize, &Entry)> {
    let entries = entry_list.get_all();

    match ui::fuzzy_select("Select an entry", entries, Some(0), false) {
        Some(index) => Some((index.clone(), entry_list.get(index))),
        None => panic!("Nothing selected"),
    }
}

fn confirm_deletion() -> Option<bool> {
    ui::confirm("Are you sure you want to delete this entry?")
}
