use dotenv::var;
use hashbrown::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use dotenv::dotenv;

const PREFIX: &str = "PROXRS_";

pub fn config() -> Arc<Mutex<HashMap<String, String>>> {
    // Load environment variables from .env file
    dotenv().ok().expect("Failed to load .env file");
    let mut conf = HashMap::new();

    // Load and parse the port
    let port =
        var(format!("{}PORT", PREFIX)).expect("Failed to load port from environment variables");
    conf.insert("port".to_string(), port);

    // Load and parse the address
    let addr =
        var(format!("{}ADDR", PREFIX)).expect("Failed to load address from environment variables");
    conf.insert("addr".to_string(), addr);

    // Load and parse login path
    let login_path = var(format!("{}LOGIN_PATH", PREFIX))
        .expect("Failed to load login path from environment variables");
    conf.insert("login_path".to_string(), login_path);

    // Load and parse the session token
    let sesion_token = var(format!("{}SESSION_TOKEN", PREFIX))
        .expect("Failed to load session token from environment variables");
    conf.insert("session_token".to_string(), sesion_token);

    // Create and return the config
    Arc::new(Mutex::new(conf))
}
