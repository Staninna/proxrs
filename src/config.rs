use hashbrown::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use dotenv::dotenv;

const ENV_PREFIX: &str = "PROXRS_";

pub fn config() -> Arc<Mutex<HashMap<String, String>>> {
    // Load environment variables from .env file
    dotenv().ok().expect("Failed to load .env file");

    // Load and parse the port
    let port = std::env::var(format!("{}PORT", ENV_PREFIX))
        .expect("Failed to load port from environment variables");

    // Load and parse the address
    let addr = std::env::var(format!("{}ADDRESS", ENV_PREFIX))
        .expect("Failed to load address from environment variables");

    // Load and parse login path
    let login_path = std::env::var(format!("{}LOGIN_PATH", ENV_PREFIX))
        .expect("Failed to load login path from environment variables");

    // Create and return the config
    let mut conf = HashMap::new();
    conf.insert("port".to_string(), port);
    conf.insert("addr".to_string(), addr);
    conf.insert("login_path".to_string(), login_path);
    Arc::new(Mutex::new(conf))
}
