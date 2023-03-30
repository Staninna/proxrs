use dotenv::var;
use hashbrown::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use dotenv::dotenv;

const PREFIX: &str = "PROXRS_";

#[derive(Hash, Eq, PartialEq)]
pub enum ConfigKey {
    Ip,
    Port,
    AuthPath,
    LoginPage,
    RenewPath,
    LoginPath,
    LogoutPath,
    StaticPath,
    SessionExpires,
    SessionCookieName,
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
        let env_value = var(format!("{}{}", PREFIX, env)).unwrap_or_else(|_| {
            panic!(
                "Failed to load {}{} from environment variables",
                PREFIX, env
            )
        });
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
    conf.set("AUTH_PATH", AuthPath).await;
    conf.set("SESSION_COOKIE_NAME", SessionCookieName).await;
    conf.set("LOGIN_PATH", LoginPath).await;
    conf.set("LOGOUT_PATH", LogoutPath).await;
    conf.set("STATIC_PATH", StaticPath).await;
    conf.set("LOGIN_PAGE", LoginPage).await;
    conf.set("SESSION_EXPIRES", SessionExpires).await;
    conf.set("RENEW_PATH", RenewPath).await;
    conf
}
