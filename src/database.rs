use super::*;

use rusqlite::{params, Connection};
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

    async fn init(&self) -> Result<(), Error> {
        // Get a connection from the pool
        let conn = self.conn().await;

        // Create the users table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    username    VARCHAR(255) NOT NULL,
                    password    VARCHAR(255) NOT NULL,
                    is_admin    INTEGER NOT NULL
                );",
            params![],
        )?;

        // Add a debug user if not exists
        conn.execute(
            "INSERT INTO users (username, password, is_admin) VALUES ('stan', 'stan', 1)
                ON CONFLICT DO NOTHING;",
            params![],
        )?;

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

    async fn conn(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().await
    }
}
