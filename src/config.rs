use confy::ConfyError;
use serde::{Serialize, Deserialize};

type ConfResult<T> = Result<T, ConfyError>;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    version: u8,
    subscription_id: u32,
    api_key: String,
}

impl Config {
    fn load() -> ConfResult<Self> {
        confy::load("tick-cli", None)
    }

    pub fn store(&self) -> ConfResult<()> {
        confy::store("tick-cli", None, self)
    }

    pub fn reset() -> ConfResult<()> {
        confy::store("tick-cli", None, Config::default())
    }

    pub fn missing_api_key(&self) -> bool {
        self.api_key.is_empty()
    }

    pub fn get_subscription_id(&self) -> &u32 {
        &self.subscription_id
    }

    pub fn get_api_key(&self) -> &String {
        &self.api_key
    }

    pub fn set_subscription_id(&mut self, subscription_id: u32) {
        self.subscription_id = subscription_id;
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }
}

pub fn load() -> ConfResult<Config> {
    Config::load()
}
