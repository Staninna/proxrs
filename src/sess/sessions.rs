use super::*;
use crate::*;

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

    pub async fn new_session(&mut self, user: String, conf: &Config, db: &Db) -> Session {
        // get expire time from config
        let expire_time = check_err!(conf.get(SessionExpireTime))
            .parse::<i64>()
            .unwrap();

        // Create a new session
        let token = Uuid::new_v4().to_string();
        let session = Session::new(user, token.clone(), expire_time, db).await;
        self.lock().await.insert(token, session.clone());

        // Return the session
        session
    }

    pub async fn validate_session_by_token(&self, token: &str, conf: &Config) -> bool {
        // Check if the session exists
        let mut locked = self.lock().await;

        let session = if let Some(session) = locked.get(token) {
            session
        } else {
            return false;
        };

        if session.is_not_expired() {
            // Get the expire time from the config
            let expire_time = check_err!(conf.get(SessionExpireTime))
                .parse::<i64>()
                .unwrap();

            // If the session is not expired, renew it
            locked.get_mut(token).unwrap().renew(expire_time);
            true
        } else {
            // If the session is expired, delete it
            locked.remove(token);
            false
        }
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

    pub async fn delete_session_by_token(&mut self, token: &str) {
        self.lock().await.remove(token);
    }
}
