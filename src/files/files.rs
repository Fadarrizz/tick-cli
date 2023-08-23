use dirs;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::result::Result;

const BASE_DIR: &str = "Tick";

#[derive(Debug)]
pub enum FileError {
    IoError(std::io::Error),
    FileError(&'static str),
}

impl FileError {
    fn new(message: &'static str) -> Self {
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

pub fn delete_documents(path: &PathBuf) -> Result<(), FileError> {
    if !ensure_path_has_base(&Dir::Document, path) {
        return Err(FileError::new("Path doesn't start with document dir"));
    }

    delete(path)
}

fn read(file: &PathBuf) -> Result<String, FileError> {
    fs::read_to_string(file).map_err(FileError::from)
}

fn write(path: &PathBuf, content: String) -> Result<(), FileError> {
    ensure_path_exists(path);

    fs::write(path, content).map_err(FileError::from)
}

fn delete(path: &PathBuf) -> Result<(), FileError> {
    fs::remove_file(path).map_err(FileError::from)
}

pub fn get_document_file_path(path: Option<&PathBuf>, child: Option<&String>) -> PathBuf {
    get_file_path(Some(&Dir::Document), path, child).unwrap()
}

pub fn get_cache_file_path(path: Option<&PathBuf>, child: Option<&String>) -> PathBuf {
    get_file_path(Some(&Dir::Cache), path, child).unwrap()
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

pub fn get_document_file_path_from(filename: &String) -> Result<PathBuf, &'static str> {
    match get_file_path(Some(&Dir::Document), None, None) {
        Err(err) => Err(err),
        Ok(mut path_buf) => {
            path_buf.push(PathBuf::from(filename));
            Ok(path_buf)
        }
    }
}

pub fn get_filename_from_path(path: &PathBuf) -> Option<String> {
    path.file_stem()
        .and_then(|s| s.to_str().map(|s| String::from(s)))
}

fn get_file_path(
    dir: Option<&Dir>,
    path: Option<&PathBuf>,
    child: Option<&String>,
) -> Result<PathBuf, &'static str> {
    match (dir, path) {
        (Some(dir), None) => {
            Ok(dir.base())
        }
        (None, Some(path)) | (Some(_), Some(path)) => {
            let mut path_buf = path.to_path_buf();

            if let Some(child) = child {
                path_buf.push(child);
            }

            Ok(path_buf)
        }
        (None, None) => Err("Either dir or path should be provided"),
    }
}

fn ensure_path_has_base(dir: &Dir, path: &PathBuf) -> bool {
    path.as_path().starts_with(dir.base())
}

fn ensure_path_exists(path: &PathBuf) {
    match fs::create_dir(path) {
        Ok(()) => (),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => (),
        Err(_) => panic!("Cannot create base dir"),
    }
}
