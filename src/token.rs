use crate::config::get_value;
use hashbrown::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Session {
    pub user: String,
    pub token: String,
    pub expires: chrono::DateTime<chrono::Local>,
}

impl Session {
    pub async fn new(
        user: String,
        conf: &Arc<Mutex<HashMap<String, String>>>,
        sessions: Arc<Mutex<HashMap<String, Session>>>,
    ) -> Option<Self> {
        // Check if user is already logged in
        let sessions = sessions.lock().await;
        for session in sessions.values() {
            if session.user == user {
                return None;
            }
        }

        // Generate a new session token
        let token = Uuid::new_v4().to_string();
        let expires_at = get_value(&conf, "session_expires").await.parse().unwrap();
        let expires = chrono::Local::now() + chrono::Duration::seconds(expires_at);

        // Return the new session
        Some(Session {
            user,
            token,
            expires,
        })
    }
}
