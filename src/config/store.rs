use super::{error::ConfigError, options::ConfigOptions};
use hashbrown::HashMap;

#[derive(Clone, Debug)]
pub struct Config {
    store: HashMap<ConfigOptions, String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: ConfigOptions) -> Result<String, ConfigError> {
        match self.store.get(&key) {
            Some(value) => Ok(value.to_string()),
            None => Err(ConfigError::MissingEnvVar(key.to_string())),
        }
    }

    pub(super) fn set(&mut self, key: ConfigOptions, value: String) {
        self.store.insert(key, value);
    }
}
