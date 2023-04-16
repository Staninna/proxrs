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
        self.store().await.insert(token, session.clone());

        // Return the session
        session
    }

    // Get the session from the store
    // TODO: Make this return an useful error
    pub async fn get(&self, token: &str) -> Option<Session> {
        // Get the sessions from the store
        let sessions = self.store().await;

        // Get the session
        match sessions.get(token) {
            Some(session) => Some(session.clone()),
            None => None,
        }
    }

    // Remove the session from the store
    // TODO: Make this return an useful error
    pub async fn delete(&mut self, session: Session) -> Result<(), ()> {
        // Get the sessions from the store
        let mut sessions = self.store().await;

        // Remove the session
        match sessions.remove(&session.token) {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }

    // Get the sessions from the store
    async fn store(&self) -> tokio::sync::MutexGuard<'_, HashMap<String, Session>> {
        self.store.lock().await
    }
}
