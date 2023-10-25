use tick_cli::{EntryList, Entry};

use crate::{ui, repository};

pub fn move_entry() -> std::io::Result<()> {
    // choose entrylist with entry to move
    let move_from_path = ui::select_file();
    let mut from_entry_list: EntryList = repository::load_entry_list(&move_from_path).expect("Cannot load entries");

    // choose entry to move
    let entry = select_entry(&mut from_entry_list).unwrap();

    // choose where to move to
    let move_to_path = ui::select_file();

    //

    Ok(())
}

fn select_entry(entry_list: &mut EntryList) -> Option<&mut Entry> {
    let entries = entry_list.get_all();

    match ui::fuzzy_select("Select an entry", entries, Some(0), false) {
        Some(index) => Some(entry_list.get_mut(index)),
        None => panic!("Nothing selected"),
    }
}
