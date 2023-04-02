use thiserror::Error;

#[derive(Error, Debug)]
pub enum SQLiteError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("SQLite error: {0}")]
    SQLite(#[from] rusqlite::Error),
}
