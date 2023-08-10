use crate::files;
use super::input;

pub fn select() -> Option<String> {
    let existing_files = files::get_document_file_names(None);

    match input::fuzzy_select("Select a date", &existing_files, Some(0), false) {
        Some(index) => Some(existing_files[index].clone()),
        None => panic!("Nothing selected"),
    }
}

fn select_file(path: Path, file: Path) -> Path {
    
}
