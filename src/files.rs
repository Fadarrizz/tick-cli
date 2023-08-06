use std::fs;
use std::io::ErrorKind;
use std::io::Result;
use std::path::PathBuf;
use dirs;

const BASE_DIR: &str = "Tick";

pub fn read(filename: &String) -> Result<String> {
    fs::read_to_string(get_file_path(filename))
}

pub fn write(filename: &String, content: String) -> Result<()> {
    ensure_base_dir_exists().unwrap();

    fs::write(get_file_path(&filename), content)
}

pub fn delete(filename: &String) -> Result<()> {
    fs::remove_file(get_file_path(filename)).expect("Cannot remove file");

    Ok(())
}

pub fn get_existing_file_names() -> Vec<String> {
    let mut file_names = fs::read_dir(get_base_dir())
        .unwrap()
        .filter_map(|file| {
            file.ok().and_then(|e| {
                e.path()
                    .file_stem()
                    .and_then(|s| s.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();

    file_names.sort_by(|a, b| b.cmp(a));

    file_names
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
