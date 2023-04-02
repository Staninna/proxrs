use crate::config::{ConfigKey::*, ConfigStore};
use chrono::{DateTime, Duration, Utc};
use hashbrown::HashMap;
use hyper::{Body, Request};
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
    pub admin: bool,
    pub user: String,
    pub token: String,
    pub expires: DateTime<Utc>,
}

impl Session {
    pub async fn new(
        user: String,
        conf: &ConfigStore,
        store: &SessionStore,
        admin: bool,
    ) -> Option<Self> {
        // Check if user is already logged in
        if let Some(session) = store.get_user(&user).await {
            if session.is_valid() {
                return None;
            }
        }

        // Generate a new session token
        let token = Uuid::new_v4().to_string();
        let session_duration = match admin {
            true => conf.get(AdminSessionDuration).await.parse::<i64>().unwrap(),
            false => conf.get(SessionDuration).await.parse::<i64>().unwrap(),
        };
        let expires = Utc::now() + Duration::seconds(session_duration);

        // Add the session to the session store
        let session = Session {
            user,
            admin,
            token,
            expires,
        };
        store.add(session.clone()).await;

        // Return the session
        Some(session)
    }

    pub async fn renew(&mut self, conf: &ConfigStore) {
        if !self.is_valid() {
            return;
        }

        let session_duratian = match self.admin {
            true => conf.get(AdminSessionDuration).await.parse::<i64>().unwrap(),
            false => conf.get(SessionDuration).await.parse::<i64>().unwrap(),
        };
        self.expires = Utc::now() + Duration::seconds(session_duratian);
    }

    pub fn is_valid(&self) -> bool {
        self.expires > Utc::now()
    }

    pub fn is_admin(&self) -> bool {
        self.admin
    }
}

pub async fn get_session_cookie(req: &Request<Body>, conf: &ConfigStore) -> Option<String> {
    // Get the session cookie name from the config
    let cookie_name = conf.get(SessionCookieName).await;

    // Get the cookie header from the request
    let cookie_header = match req.headers().get("Cookie") {
        Some(cookie_header) => cookie_header,
        None => return None,
    };

    // Get the session token from the cookie header
    match cookie_header.to_str() {
        Ok(cookie_header) => {
            // Split the cookie header into individual cookies
            let cookies: Vec<&str> = cookie_header.split(';').collect();

            // Find the session cookie
            let session_cookie = cookies
                .iter()
                .find(|cookie| cookie.starts_with(&cookie_name));

            // Get the session token from the session cookie
            match session_cookie {
                Some(session_cookie) => {
                    let session_token = session_cookie.split('=').collect::<Vec<&str>>()[1];
                    Some(session_token.to_string())
                }
                None => None,
            }
        }
        Err(_) => None,
    }
}
