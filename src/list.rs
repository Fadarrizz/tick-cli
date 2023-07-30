use tick_cli::EntryList;
use crate::{files, input};

pub fn list_entries() -> std::io::Result<()> {
    let filename = select_file().unwrap();
    let entries: EntryList = files::load_entry_list(&filename).expect("Cannot load entries");

    print!("{}", entries);

    Ok(())
}

fn select_file() -> Option<String> {
    let existing_files = files::get_existing_file_names();

    match input::fuzzy_select("Select a date", &existing_files, Some(0), false) {
        Some(index) => Some(existing_files[index].clone()),
        None => panic!("Nothing selected"),
    }
}
