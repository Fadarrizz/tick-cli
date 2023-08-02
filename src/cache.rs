use std::{collections::HashMap, path::PathBuf, fs};
use serde::{Serialize, Deserialize};

const FILENAME: &str = "cache.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiCacheValue {
    validator: String,
    value: String,
}

impl ApiCacheValue {
    pub fn new(validator: String, value: String) -> Self
    {
        Self { validator, value }
    }
}

pub struct Cache {
    map: HashMap<String, ApiCacheValue>,
}

impl Cache {
    pub fn new() -> Self {
        let map = match fs::read_to_string(get_file_path()) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| HashMap::new()), 
            Err(_) => HashMap::new()
        };

        Cache { map }
    }

    pub fn get(&self, key: String) -> Option<ApiCacheValue> {
        self.map.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, value: ApiCacheValue) {
        self.map.insert(key, value);
        let content = serde_json::to_string(&self.map).unwrap();
        fs::write(get_file_path(), content).expect("Failed to write to cache file");
    }
}

fn get_file_path() -> PathBuf {
    let mut path = dirs::cache_dir().unwrap();

    path.push(FILENAME);

    path
}
