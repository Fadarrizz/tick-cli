use std::path::PathBuf;

use crate::files;
use super::input;

pub fn select_file() -> PathBuf {
    let mut path = files::get_document_file_path(None, None);

    // Year
    path = files::get_document_file_path(
        Some(path),
        Some(&select(files::get_file_names(&path))),
    );

    // Month
    path = files::get_document_file_path(
        Some(path),
        Some(&select(files::get_file_names(&path))),
    );

    // File
    path = files::get_document_file_path(
        Some(path),
        Some(&select(files::get_file_names(&path))),
    );

    path
}

fn select(file_names: Vec<String>) -> String {
    match input::fuzzy_select("Select a date", &file_names, Some(0), false) {
        Some(index) => file_names[index].clone(),
        None => panic!("Nothing selected"),
    }
}
