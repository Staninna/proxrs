use crate::*;

use chrono::{DateTime, Duration, Utc};

#[derive(Clone, Debug)]
pub struct Session {
    pub user: String,
    pub token: String,
    pub expire_time: DateTime<Utc>,
}

impl Session {
    pub fn new(user: String, token: String, expire_time: i64) -> Self {
        let expire_time = Utc::now() + chrono::Duration::from(Duration::seconds(expire_time));

        Self {
            user,
            token,
            expire_time,
        }
    }

    pub(super) fn is_not_expired(&self) -> bool {
        self.expire_time > Utc::now()
    }

    pub fn renew(&mut self, expire_time: i64) {
        self.expire_time = Utc::now() + chrono::Duration::from(Duration::seconds(expire_time));
    }
}
