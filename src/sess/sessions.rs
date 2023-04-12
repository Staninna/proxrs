use super::*;
use hashbrown::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Sessions {
    store: Arc<Mutex<HashMap<String, Session>>>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn lock(&self) -> tokio::sync::MutexGuard<'_, HashMap<String, Session>> {
        self.store.lock().await
    }

    pub async fn new_session(&mut self, user: String) -> Session {
        // Create a new session
        let token = Uuid::new_v4().to_string();
        let session = Session::new(user, token.clone());
        self.lock().await.insert(token, session.clone());

        // Return the session
        session
    }

    pub async fn validate_session_by_token(&self, token: &str) -> bool {
        // Check if the session exists
        if let Some(_) = self.lock().await.get(token) {
            // If the session exists, return true
            return true;
        }

        // If the session doesn't exist, return false
        false
    }

    pub async fn get_user_by_token(&self, token: &str) -> String {
        // Check if the session exists
        if let Some(session) = self.lock().await.get(token) {
            // If the session exists, return the user
            return session.user.clone();
        }

        // If the session doesn't exist, return None
        String::from("Unknown")
    }
}
