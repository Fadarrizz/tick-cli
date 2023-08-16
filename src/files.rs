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
    read(get_file_path(&Dir::Document , None, Some(filename)))
}

pub fn read_from_cache(filename: &String) -> Result<String> {
    read(get_file_path(&Dir::Cache , None, Some(filename)))
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

fn read(file: PathBuf) -> Result<String> {
    fs::read_to_string(file)
}

fn write(path: PathBuf, content: String) -> Result<()> {
    ensure_path_exists(&path).unwrap();

    fs::write(get_file_path(dir, None, Some(filename)), content)
}

fn delete(dir: &Dir, filename: &String) -> Result<()> {
    fs::remove_file(get_file_path(dir, None, Some(filename)))
}

pub fn get_document_file_path(path: Option<PathBuf>, child: Option<&String>) -> PathBuf {
    get_file_path(Some(&Dir::Document), path, child).unwrap()
}

pub fn get_file_names(path: &PathBuf) -> Vec<String> {
    let mut file_names = fs::read_dir(path)
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

fn get_file_path(dir: Option<&Dir>, path: Option<PathBuf>, child: Option<&String>) -> Result<PathBuf> {
    let mut path_buf;

    match (dir, path) {
        (Some(dir), None) => {
            path_buf = dir.base();
            Ok(())
        },
        (None, Some(path)) => {
            path_buf = path;
            Ok(())
        },
        (Some(dir), Some(path)) => {
            path_buf = path;
            Ok(())
        },
        (None, None) => Err("Either dir or path should be provided"),
    };

    if let Some(child) = child {
        path_buf.push(child);
    }

    if path_buf.as_path().is_file() {
        path_buf.set_extension("json");
    }

    Ok(path_buf)
}

fn ensure_path_exists(path: &PathBuf) -> Result<()> {
    match fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(_) => panic!("Cannot create base dir"),
    }
}

fn path_by_file_name(file_name: &String) -> PathBuf {
    
}
