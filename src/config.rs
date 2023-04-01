use dotenv::dotenv;
use dotenv::var;
use hashbrown::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

const PREFIX: &str = "PROXRS_";

#[derive(Hash, Eq, PartialEq)]
pub enum ConfigKey {
    Ip,
    Port,
    StaticDir,
    SessionDuration,
    TemplateDir,
    InternalErrorPage,
    SessionCookieName,
    SpecialRouteEndpoint,
}

#[derive(Clone)]
pub struct ConfigStore {
    pub config: Arc<Mutex<HashMap<ConfigKey, String>>>,
}

impl ConfigStore {
    pub fn new() -> Self {
        ConfigStore {
            config: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: ConfigKey) -> String {
        let config = self.config.lock().await;
        config.get(&key).unwrap().clone()
    }

    async fn set(&self, env: &str, key: ConfigKey) {
        let env_value = var(format!("{}{}", PREFIX, env))
            .unwrap_or_else(|_| panic!("Failed to load {}{}", PREFIX, env));
        let mut config = self.config.lock().await;
        config.insert(key, env_value);
    }
}

pub async fn config() -> ConfigStore {
    use ConfigKey::*;

    // Load environment variables from .env file
    dotenv().expect("Failed to load .env file");
    let conf = ConfigStore::new();
    conf.set("IP", Ip).await;
    conf.set("PORT", Port).await;
    conf.set("STATIC_DIR", StaticDir).await;
    conf.set("TEMPLATE_DIR", TemplateDir).await;
    conf.set("SESSION_DURATION", SessionDuration).await;
    conf.set("SESSION_COOKIE_NAME", SessionCookieName).await;
    conf.set("INTERNAL_ERROR_PAGE", InternalErrorPage).await;
    conf.set("SPECIAL_ROUTE_ENDPOINT", SpecialRouteEndpoint)
        .await;

    conf
}
