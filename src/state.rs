use crate::*;

use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    sessions: Sessions,
    client: Client,
    conf: Config,
    tera: Tera,
    db: Db,
}

impl AppState {
    pub async fn new(conf: &Config) -> Self {
        // Initialize the database
        let db_file = check_err!(conf.get(DbFile));
        let db = check_err!(Db::new(db_file).await);

        // Initialize the sessions
        let sessions = Sessions::new();

        // Initialize the template engine
        let static_path = check_err!(conf.get(StaticDir));
        let tera = check_err!(Tera::new(&format!("{}/**/*", static_path)));

        // Create the client
        let client = hyper::Client::builder().build(HttpsConnector::new());

        // Clone the config
        let conf = conf.clone();

        Self {
            sessions,
            client,
            conf,
            tera,
            db,
        }
    }

    pub fn extract(&self) -> (Sessions, Client, Config, Tera, Db) {
        (
            self.sessions.clone(),
            self.client.clone(),
            self.conf.clone(),
            self.tera.clone(),
            self.db.clone(),
        )
    }
}
