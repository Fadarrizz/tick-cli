use std::{io::ErrorKind, path::PathBuf};
use tick_cli::EntryList;
use crate::files::{self, FileError};

pub fn load_entry_list(path: &PathBuf) -> Result<EntryList, FileError> {
    match files::read_from_documents(path) {
        Ok(data) => {
            Ok(serde_json::from_str(data.as_str()).expect("Unable to parse file to json"))
        },
        Err(error) => match error {
            FileError::FileError(_) => Err(error),
            FileError::IoError(io_err) => {
                if io_err.kind() == ErrorKind::NotFound {
                    Ok(EntryList::empty())
                } else {
                    panic!("Problem opening the file: {:?}", io_err);
                }
            }
        }
    }
}

pub fn store_entry_list(entries: &EntryList, path: &PathBuf) -> Result<(), ErrorKind> {
    files::write_to_documents(
        path,
        serde_json::to_string_pretty(&entries).expect("Cannot serialize entries"),
    )
    .expect("Cannot write to file");

    Ok(())
}

pub fn delete_entry_list(path: &PathBuf) -> Result<(), ErrorKind> {
    files::delete_documents(path).expect("Unable to delete file");
    
    Ok(())
}
