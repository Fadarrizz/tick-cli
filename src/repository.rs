use std::{io::ErrorKind, path::PathBuf};
use tick_cli::EntryList;
use crate::files::{self, FileError};

pub fn load_entry_list(path: PathBuf) -> Result<EntryList, FileError> {
    let data = files::read_from_documents(&path).unwrap_or_else(|error| {
        if let FileError::FileError(error) = error {
            return Err(error);
        };

        match error {
            FileError::IoError(error) => Ok(l),
            FileError::FileError(error) => Err(error),
            _ => panic!("Problem opening the file: {:?}", error),

        }

        if error.kind() == ErrorKind::NotFound {
            String::new()
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    if data.is_empty() {
        return Ok(EntryList::empty());
    }

    Ok(serde_json::from_str(data.as_str()).expect("Unable to parse file to json"))
}

pub fn store_entry_list(entries: EntryList, filename: &String) -> Result<(), ErrorKind> {
    files::write_to_documents(
        filename,
        serde_json::to_string_pretty(&entries).expect("Cannot serialize entries"),
    )
    .expect("Cannot write to file");

    Ok(())
}

pub fn delete_entry_list(filename: &String) -> Result<(), ErrorKind> {
    files::delete_documents(filename).expect("Unable to delete file");
    
    Ok(())
}
