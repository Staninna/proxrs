use super::*;
use crate::error::Error;
use hashbrown::HashMap;

#[derive(Clone, Debug)]
pub struct ConfigStore {
    store: HashMap<ConfigOptions, String>,
}

impl ConfigStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: ConfigOptions) -> Result<String, Error> {
        match self.store.get(&key) {
            Some(value) => Ok(value.to_string()),
            None => Err(Error::MissingConfigVar(key.to_string())),
        }
    }

    pub(super) fn set(&mut self, key: ConfigOptions, value: String) {
        self.store.insert(key, value);
    }
}
