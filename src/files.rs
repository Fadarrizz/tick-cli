use std::fs;
use std::io::ErrorKind;
use std::io::Result;
use std::path::PathBuf;
use dirs;

const BASE_DIR: &str = "Tick";

enum Dir {
    Document,
    Cache,
}

impl Dir {
    fn base(&self) -> PathBuf {
        let mut path = match self {
            Self::Document => dirs::document_dir().expect("Default document dir not found"),
            Self::Cache => dirs::cache_dir().expect("Default cache dir not found"),
        };

        path.push(BASE_DIR);

        path
    }
}

pub fn read_from_documents(filename: &String) -> Result<String> {
    read(&Dir::Document, filename)
}

pub fn read_from_cache(filename: &String) -> Result<String> {
    read(&Dir::Cache, filename)
}

pub fn write_to_documents(filename: &String, content: String) -> Result<()> {
    write(&Dir::Document, filename, content)
}

pub fn write_to_cache(filename: &String, content: String) -> Result<()> {
    write(&Dir::Cache, filename, content)
}

pub fn delete_documents(filename: &String) -> Result<()> {
    delete(&Dir::Document, filename)
}

fn read(dir: &Dir, filename: &String) -> Result<String> {
    fs::read_to_string(get_file_path(dir, filename))
}

fn write(dir: &Dir, filename: &String, content: String) -> Result<()> {
    ensure_base_dir_exists(dir).unwrap();

    fs::write(get_file_path(dir, filename), content)
}

fn delete(dir: &Dir, filename: &String) -> Result<()> {
    fs::remove_file(get_file_path(dir, filename)).expect("Cannot remove file");

    Ok(())
}

pub fn get_document_file_names() -> Vec<String> {
    get_existing_file_names(&Dir::Document)
}

fn get_existing_file_names(dir: &Dir) -> Vec<String> {
    let mut file_names = fs::read_dir(dir.base())
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

fn get_file_path(dir: &Dir, filename: &String) -> PathBuf {
    let mut path = dir.base();

    path.push(filename);
    path.set_extension("json");

    path
}

fn ensure_base_dir_exists(dir: &Dir) -> Result<()> {
    match fs::create_dir(dir.base()) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(_) => panic!("Cannot create base dir"),
    }
}
