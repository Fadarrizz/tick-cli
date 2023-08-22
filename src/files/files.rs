use std::fs;
use std::io::ErrorKind;
use std::result::Result;
use std::path::PathBuf;
use dirs;

const BASE_DIR: &str = "Tick";

#[derive(Debug)]
pub enum FileError {
    IoError(std::io::Error),
    FileError(&'static str),
}

impl FileError {
    fn new(message: &str) -> Self {
        FileError::FileError(message)
    }
}

impl From<std::io::Error> for FileError {
    fn from(error: std::io::Error) -> Self {
        FileError::IoError(error)
    }
}

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

pub fn read_from_documents(path: &PathBuf) -> Result<String, FileError> {
    if !ensure_path_has_base(&Dir::Document, path) {
        return Err(FileError::new("Path doesn't start with document dir"));
    }

    read(path)
}

pub fn read_from_cache(path: &PathBuf) -> Result<String, FileError> {
    if !ensure_path_has_base(&Dir::Cache, path) {
        return Err(FileError::new("Path doesn't start with cache dir"));
    }

    read(path)
}

pub fn write_to_documents(path: &PathBuf, content: String) -> Result<(), FileError> {
    if !ensure_path_has_base(&Dir::Document, path) {
        return Err(FileError::new("Path doesn't start with document dir"));
    }

    write(path, content)
}

pub fn write_to_cache(path: &PathBuf, content: String) -> Result<(), FileError> {
    if !ensure_path_has_base(&Dir::Cache, path) {
        return Err(FileError::new("Path doesn't start with cache dir"));
    }

    write(path, content)
}

pub fn delete_documents(path: PathBuf) -> Result<(), FileError> {
    if !ensure_path_has_base(&Dir::Document, &path) {
        return Err(FileError::new("Path doesn't start with document dir"));
    }

    delete(path)
}

fn read(file: &PathBuf) -> Result<String, FileError> {
    fs::read_to_string(file).map_err(FileError::from)
}

fn write(path: &PathBuf, content: String) -> Result<(), FileError> {
    ensure_path_exists(&path).is_ok();

    fs::write(path, content).map_err(FileError::from)
}

fn delete(path: PathBuf) -> Result<(), FileError> {
    fs::remove_file(path).map_err(FileError::from)
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

fn get_file_path(dir: Option<&Dir>, path: Option<PathBuf>, child: Option<&String>) -> Result<PathBuf, &'static str> {
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

fn ensure_path_has_base(dir: &Dir, path: &PathBuf) -> bool {
    path.as_path().starts_with(dir.base())
}

fn ensure_path_exists(path: &PathBuf) -> Result<(), FileError> {
    match fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(_) => panic!("Cannot create base dir"),
    }
}
