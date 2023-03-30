use dotenv::var;
use hashbrown::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use dotenv::dotenv;

const PREFIX: &str = "PROXRS_";

#[derive(Clone)]
pub struct ConfigStore {
    pub config: Arc<Mutex<HashMap<String, String>>>,
}

impl ConfigStore {
    pub fn new() -> Self {
        ConfigStore {
            config: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: &str) -> String {
        let config = self.config.lock().await;
        config.get(key).unwrap().clone()
    }

    async fn set(&self, env: &str, key: &str) {
        let env_value = var(format!("{}{}", PREFIX, env)).expect(
            format!(
                "Failed to load {}{} from environment variables",
                PREFIX, env
            )
            .as_str(),
        );
        let mut config = self.config.lock().await;
        config.insert(key.to_string(), env_value);
    }
}

pub async fn config() -> ConfigStore {
    // Load environment variables from .env file
    dotenv().ok().expect("Failed to load .env file");
    let conf = ConfigStore::new();
    conf.set("ADDRESS", "address").await;
    conf.set("PORT", "port").await;
    conf.set("AUTH_PATH", "auth_path").await;
    conf.set("SESSION_COOKIE_NAME", "session_cookie_name").await;
    conf.set("LOGIN_PATH", "login_path").await;
    conf.set("LOGOUT_PATH", "logout_path").await;
    conf.set("STATIC_PATH", "static_path").await;
    conf.set("LOGIN_PAGE", "login_page").await;
    conf.set("SESSION_EXPIRES", "session_expires").await;
    conf.set("RENEW_PATH", "renew_path").await;
    conf
}
