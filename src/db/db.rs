use super::error::SQLiteError;
use rusqlite::{params, Connection};
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

impl Db {
    pub async fn new(file: String) -> Result<Self, SQLiteError> {
        // Create the database
        let db = Db {
            conn: Arc::new(Mutex::new(Connection::open(file).unwrap())),
        };

        // Initialize the database
        db.init().await?;

        // Return the database
        Ok(db)
    }

    async fn init(&self) -> Result<(), SQLiteError> {
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

        // Everything went well
        Ok(())
    }

    async fn conn(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().await
    }
}
