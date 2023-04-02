use crate::{config::error::ConfigError, db::error::SQLiteError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // Sqlite
    #[error("{0}")]
    SQLite(#[from] SQLiteError),

    // Config
    #[error("{0}")]
    Config(#[from] ConfigError),
}
