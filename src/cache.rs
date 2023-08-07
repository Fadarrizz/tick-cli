use std::collections::HashMap;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use crate::files;

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
        let map = match files::read_from_cache(&FILENAME.to_string()) {
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

        files::write_to_cache(
            &FILENAME.to_string(),
            serde_json::to_string(&self.map).unwrap(),
        ).expect("Failed to write to cache file");
    }
}
