use crate::config::ConfigStore;
use hashbrown::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionStore {
    pub sessions: Arc<Mutex<HashMap<String, Session>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        SessionStore {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add(&self, session: Session) {
        let mut sessions = self.sessions.lock().await;
        sessions.insert(session.token.clone(), session);
    }

    pub async fn get_token(&self, token: &str) -> Option<Session> {
        let sessions = self.sessions.lock().await;
        sessions.get(token).cloned()
    }

    pub async fn get_user(&self, user: &str) -> Option<Session> {
        let sessions = self.sessions.lock().await;
        sessions.values().find(|s| s.user == user).cloned()
    }

    pub async fn remove(&self, token: &str) {
        let mut sessions = self.sessions.lock().await;
        sessions.remove(token);
    }
}

#[derive(Clone, Debug)]
pub struct Session {
    pub user: String,
    pub token: String,
    pub expires: chrono::DateTime<chrono::Local>,
}

impl Session {
    pub async fn new(user: String, conf: &ConfigStore, store: &SessionStore) -> Option<Self> {
        // Check if user is already logged in
        if let Some(session) = store.get_user(&user).await {
            if session.is_valid() {
                return None;
            }
        }

        // Generate a new session token
        let token = Uuid::new_v4().to_string();
        let expires_at = conf.get("session_expires").await.parse::<i64>().unwrap();
        let expires = chrono::Local::now() + chrono::Duration::seconds(expires_at);

        // Return the new session
        Some(Session {
            user,
            token,
            expires,
        })
    }

    pub fn is_valid(&self) -> bool {
        self.expires > chrono::Local::now()
    }
}
