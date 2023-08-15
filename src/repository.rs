use std::io::ErrorKind;
use tick_cli::EntryList;
use crate::files;

// pub fn get_entry_lists_by_filename() -> Vec<String> {
//     files::get_document_file_names()
// }

pub fn load_entry_list(filename: &String) -> Result<EntryList, ErrorKind> {
    let data = files::read_from_documents(filename).unwrap_or_else(|error| {
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
