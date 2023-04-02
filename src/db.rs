use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

impl Db {
    pub async fn new(db: &str) -> Self {
        // Initialize the database
        let mut db = Db {
            conn: Arc::new(Mutex::new(Connection::open(db).unwrap())),
        };
        db.init().await.unwrap();

        // Return the database
        db
    }

    pub async fn init(&mut self) -> rusqlite::Result<()> {
        // Create a table with users and their passwords hashes
        self.conn.lock().await.execute(
            "CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    username VARCHAR(255) NOT NULL UNIQUE,
                    password VARCHAR(255) NOT NULL
            );",
            [],
        )?;

        // Everything went fine
        Ok(())
    }

    pub async fn validate_user(&self, user: &User) -> bool {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare("SELECT * FROM users WHERE username = ? AND password = ?")
            .unwrap();

        let rows = stmt.query_map(params![user.username, user.password], |row| {
            Ok(User {
                username: row.get(1)?,
                password: row.get(2)?,
            })
        });

        match rows {
            Ok(rows) => rows.count() == 1,
            Err(_) => false,
        }
    }
}
