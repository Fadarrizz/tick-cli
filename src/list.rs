use crate::{files, input, repository};

pub fn list_entries() -> std::io::Result<()> {
    let filename = select_file().unwrap();
    let entries = repository::load_entry_list(&filename).expect("Cannot load entries");

    print!("{}", entries);

    Ok(())
}

fn select_file() -> Option<String> {
    let existing_files = files::get_document_file_names();

    match input::fuzzy_select("Select a date", &existing_files, Some(0), false) {
        Some(index) => Some(existing_files[index].clone()),
        None => panic!("Nothing selected"),
    }
}
