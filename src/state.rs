use crate::*;

#[derive(Clone)]
pub struct AppState {
    sessions: Sessions,
    client: Client,
    conf: Config,
    db: Db,
}

impl AppState {
    pub async fn new(conf: &Config) -> Self {
        // Initialize the database
        let db_file = check_err!(conf.get(DbFile));
        let db = check_err!(Db::new(db_file).await);

        // Initialize the sessions
        let sessions = Sessions::new();

        // Create the client
        let client = hyper::Client::builder().build(HttpsConnector::new());

        // Clone the config
        let conf = conf.clone();

        Self {
            sessions,
            client,
            conf,
            db,
        }
    }

    pub fn extract(&self) -> (Sessions, Client, Config, Db) {
        (
            self.sessions.clone(),
            self.client.clone(),
            self.conf.clone(),
            self.db.clone(),
        )
    }
}
