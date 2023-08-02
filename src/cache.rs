use std::{collections::HashMap, path::PathBuf, fs, io::ErrorKind};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

const FILENAME: &str = "cache.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct CachedResponse<T> {
    etag: Option<String>,
    last_modified: Option<String>,
    json: T,
}

impl<T> CachedResponse<T> {
    pub fn new(etag: Option<String>, last_modified: Option<String>, json: T) -> Self {
        Self { etag, last_modified, json }
    }

    pub fn get_etag(&self) -> Option<&String> {
        self.etag.as_ref()
    }

    pub fn get_last_modified(&self) -> Option<&String> {
        self.last_modified.as_ref()
    }

    pub fn get_json(&self) -> &T {
        &self.json
    }
}

pub struct Cache<T> {
    map: HashMap<String, CachedResponse<T>>,
}

impl<T: DeserializeOwned + Serialize + Clone> Cache<T> {
    pub fn new() -> Self {
        let map = match fs::read_to_string(get_cache_path()) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| HashMap::new()), 
            Err(_) => HashMap::new()
        };

        Cache { map }
    }

    pub fn get(&self, key: String) -> Option<CachedResponse<T>> {
        self.map.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, cached_response: CachedResponse<T>) {
        self.map.insert(key, cached_response);

        ensure_base_dir_exists().unwrap();

        fs::write(
            get_cache_path(),
            serde_json::to_string(&self.map).unwrap(),
        ).expect("Failed to write to cache file");
    }
}

fn get_cache_path() -> PathBuf {
    let mut path = get_base_dir();

    path.push(FILENAME);

    path
}

fn get_base_dir() -> PathBuf {

    let mut path = dirs::cache_dir().unwrap();

    path.push("Tick");

    path
}

fn ensure_base_dir_exists() -> Result<(), ErrorKind> {
    match fs::create_dir(get_base_dir()) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(_) => panic!("Cannot create base dir"),
    }
}
