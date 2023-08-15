use crate::files::{self, get_document_file_path};
use super::input;

pub fn select_file() -> String {
    let mut path = get_document_file_path(None);
    let mut selected = String::new();


    while !files::is_file(&path) {
        let existing_files = files::get_existing_file_names(&path);

        selected = match input::fuzzy_select("Select a date", &existing_files, Some(0), false) {
            Some(index) => existing_files[index].clone(),
            None => panic!("Nothing selected"),
        };

        path.push(selected.clone());

        dbg!(&path);
    }

    selected
}
