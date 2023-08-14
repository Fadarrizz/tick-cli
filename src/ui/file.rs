use std::path::PathBuf;
use crate::files::{self, get_document_file_path};
use super::input;

pub fn select_file() -> PathBuf {
    let mut path = get_document_file_path(None);

    while !files::is_file(&path) {
        let existing_files = files::get_document_file_names(Some(&path));

        let selected = match input::fuzzy_select("Select a date", &existing_files, Some(0), false) {
            Some(index) => Some(&existing_files[index]),
            None => panic!("Nothing selected"),
        };

        path = files::get_document_file_path(selected);
    }

    path
}
