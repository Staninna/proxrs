use super::*;

use rusqlite::{params, Connection};
use sha2::Digest;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

impl Db {
    pub async fn new(file: String) -> Result<Self, Error> {
        // Create the database
        let db = Db {
            conn: Arc::new(Mutex::new(Connection::open(file)?)),
        };

        // Initialize the database
        db.init().await?;

        // Return the database
        Ok(db)
    }

    pub async fn validate_user(&self, username: &str, password: &str) -> Result<bool, Error> {
        // Get a connection
        let conn = self.conn().await;

        // Do the query
        let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ? AND password = ?;")?;
        let mut rows = stmt.query(params![username, password])?;

        // Return if the user exists
        Ok(rows.next()?.is_some())
    }

    async fn init(&self) -> Result<(), Error> {
        // Get a connection from the pool
        let conn = self.conn().await;

        // Create the users table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    username    VARCHAR(255) NOT NULL,
                    password    VARCHAR(255) NOT NULL,
                    admin       INTEGER NOT NULL
                );",
            params![],
        )?;

        // Get all users
        let mut stmt = conn.prepare("SELECT * FROM users;")?;
        let mut rows = stmt.query(params![])?;

        // If there are no users, create the default admin user
        if rows.next()?.is_none() {
            // Hash the passwords
            let stan_pass = "stan";
            let mut hasher = sha2::Sha256::new();
            hasher.update(stan_pass);
            let stan_pass = hex::encode(hasher.finalize());

            let admin_pass = "admin";
            let mut hasher = sha2::Sha256::new();
            hasher.update(admin_pass);
            let admin_pass = hex::encode(hasher.finalize());

            // Insert the users
            conn.execute(
                "INSERT INTO users (username, password, admin) VALUES (?, ?, ?), (?, ?, ?);",
                params!["stan", stan_pass, 1, "admin", admin_pass, 0],
            )?;
        }

        // Create the proxy table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS proxy (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    name        VARCHAR(255) NOT NULL,
                    host        VARCHAR(255) NOT NULL,
                    port        INTEGER NOT NULL,
                    is_enabled  INTEGER NOT NULL DEFAULT 1
                );",
            params![],
        )?;

        // Everything went well
        Ok(())
    }

    pub async fn is_admin(&self, username: &str) -> Result<bool, Error> {
        // Get a connection
        let conn = self.conn().await;

        // Do the query
        let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ? AND admin = 1;")?;
        let mut rows = stmt.query(params![username])?;

        // Return if the user exists
        Ok(rows.next()?.is_some())
    }

    async fn conn(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().await
    }
}
