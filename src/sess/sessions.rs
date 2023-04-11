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
        let token = Uuid::new_v4().to_string();
        let session = Session::new(user, token.clone());
        self.lock().await.insert(token, session.clone());
        session
    }

    pub async fn get_session_by_user(&self, user: &str) -> Option<Session> {
        for (_, session) in self.lock().await.iter() {
            if session.user == user {
                return Some(session.clone());
            }
        }
        None
    }
}
