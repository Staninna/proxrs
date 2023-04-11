use super::*;
use hashbrown::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Sessions {
    store: HashMap<String, Session>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn new_session(&mut self, user: String) -> Session {
        let token = Uuid::new_v4().to_string();
        let session = Session::new(user, token.clone());
        self.store.insert(token, session.clone());
        session
    }

    pub fn get_session_by_user(&self, user: &str) -> Option<Session> {
        for (_, session) in self.store.iter() {
            if session.user == user {
                return Some(session.clone());
            }
        }
        None
    }
}
