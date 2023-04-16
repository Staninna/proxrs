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

    pub async fn new_session(&mut self, user: String, conf: &Config, db: &Db) -> Session {
        // Get expire time from config
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

    pub async fn get(&self, token: &str) -> Option<Session> {
        let sessions = self.lock().await;
        match sessions.get(token) {
            Some(session) => Some(session.clone()),
            None => None,
        }
    }

    pub async fn del(&mut self, token: &str) -> Result<(), ()> {
        let mut sessions = self.lock().await;
        match sessions.remove(token) {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }

    async fn lock(&self) -> tokio::sync::MutexGuard<'_, HashMap<String, Session>> {
        self.store.lock().await
    }
}
