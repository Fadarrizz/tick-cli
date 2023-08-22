use std::path::PathBuf;

use crate::files;
use super::input;

pub fn select_file() -> PathBuf {
    let mut path = files::get_document_file_path(None, None);

    // Year
    path = files::get_document_file_path(
        Some(&path),
        Some(&select(files::get_file_names(&path), "Select year")),
    );

    // Month
    path = files::get_document_file_path(
        Some(&path),
        Some(&select(files::get_file_names(&path), "Select month")),
    );

    // File
    path = files::get_document_file_path(
        Some(&path),
        Some(&select(files::get_file_names(&path), "Select date")),
    );

    path
}

fn select(file_names: Vec<String>, prompt: &str) -> String {
    match input::fuzzy_select(prompt, &file_names, Some(0), false) {
        Some(index) => file_names[index].clone(),
        None => panic!("Nothing selected"),
    }
}
