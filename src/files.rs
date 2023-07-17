use std::fs;
use std::io::ErrorKind;
use std::io::Result;
use std::path::PathBuf;
use tick_cli::EntryList;

extern crate dirs;

const BASE_DIR: &str = "Tick";

pub fn load_entry_list(filename: &String) -> Result<EntryList> {
    let data = fs::read_to_string(get_file_path(&filename)).unwrap_or_else(|error| {
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

pub fn store_entry_list(entries: EntryList, filename: &String) -> Result<()> {
    ensure_base_dir_exists().unwrap();

    fs::write(
        get_file_path(&filename),
        serde_json::to_string_pretty(&entries).expect("Cannot serialize entries"),
    )
    .expect("Cannot write to file");

    Ok(())
}

pub fn get_existing_file_names() -> Vec<String> {
    fs::read_dir(get_base_dir())
        .unwrap()
        .filter_map(|file| {
            file.ok().and_then(|e| {
                e.path()
                    .file_stem()
                    .and_then(|s| s.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>()
}

fn get_file_path(filename: &String) -> PathBuf {
    let mut path = get_base_dir();

    path.push(filename);
    path.set_extension("json");

    path
}

fn get_base_dir() -> PathBuf {
    let mut path: PathBuf = dirs::document_dir().unwrap();

    path.push(BASE_DIR);

    path
}

fn ensure_base_dir_exists() -> Result<()> {
    match fs::create_dir(get_base_dir()) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(_) => panic!("Cannot create base dir"),
    }
}
