#[derive(Clone, Debug)]
pub struct Session {
    pub user: String,
    pub token: String,
}

impl Session {
    pub fn new(user: String, token: String) -> Self {
        Self { user, token }
    }
}
