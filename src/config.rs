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
    let addr = var(format!("{}ADDRESS", PREFIX))
        .expect("Failed to load address from environment variables");
    conf.insert("address".to_string(), addr);

    // Load and parse auth path
    let auth_path = var(format!("{}AUTH_PATH", PREFIX))
        .expect("Failed to load login path from environment variables");
    conf.insert("auth_path".to_string(), auth_path);

    // Load and parse the session token
    let sesion_token = var(format!("{}SESSION_COOKIE_NAME", PREFIX))
        .expect("Failed to load session token from environment variables");
    conf.insert("session_cookie_name".to_string(), sesion_token);

    // Load and parse the login path
    let login_path = var(format!("{}LOGIN_PATH", PREFIX))
        .expect("Failed to load login path from environment variables");
    conf.insert("login_path".to_string(), login_path);

    // Load and parse the logout path
    let logout_path = var(format!("{}LOGOUT_PATH", PREFIX))
        .expect("Failed to load logout path from environment variables");
    conf.insert("logout_path".to_string(), logout_path);

    // Load and parse the static path
    let static_path = var(format!("{}STATIC_PATH", PREFIX))
        .expect("Failed to load static path from environment variables");
    conf.insert("static_path".to_string(), static_path);

    // Load and parse the login page
    let login_page = var(format!("{}LOGIN_PAGE", PREFIX))
        .expect("Failed to load login page from environment variables");
    conf.insert("login_page".to_string(), login_page);

    // Load and parse the session expires
    let session_expires = var(format!("{}SESSION_EXPIRES", PREFIX))
        .expect("Failed to load session expires from environment variables");
    conf.insert("session_expires".to_string(), session_expires);

    // Create and return the config
    Arc::new(Mutex::new(conf))
}

pub async fn get_value(conf: &Arc<Mutex<HashMap<String, String>>>, key: &str) -> String {
    conf.lock().await.get(key).unwrap().to_string()
}
