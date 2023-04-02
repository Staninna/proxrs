use super::options::ConfigOptions;
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

    pub fn get(&self, key: ConfigOptions) -> Option<&String> {
        self.store.get(&key)
    }

    pub(super) fn set(&mut self, key: ConfigOptions, value: String) {
        self.store.insert(key, value);
    }
}
