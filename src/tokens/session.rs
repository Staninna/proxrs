use crate::*;

use chrono::{DateTime, Duration, Utc};

#[derive(Clone, Debug)]
pub struct Session {
    pub admin: bool,
    pub user: String,
    pub token: String,
    pub expire_time: DateTime<Utc>,
}

impl Session {
    pub async fn new(user: String, token: String, expire_time: i64, db: &Db) -> Self {
        let expire_time = Utc::now() + chrono::Duration::from(Duration::seconds(expire_time));

        // Check if the user is an admin
        let admin = db.is_admin(&user).await.unwrap_or(false);

        Self {
            user,
            admin,
            token,
            expire_time,
        }
    }

    pub fn expired(&self) -> bool {
        Utc::now() > self.expire_time
    }

    pub fn renew(&mut self, expire_time: i64) {
        self.expire_time = Utc::now() + chrono::Duration::from(Duration::seconds(expire_time));
    }
}
