use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // Database error
    #[error("Database error: {0}")]
    SQLite(#[from] rusqlite::Error),
}
