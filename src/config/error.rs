use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to load `.env` file: {0}")]
    DotEnvError(#[from] dotenv::Error),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Empty value in the config store for key: {0}")]
    EmptyEnvVar(String),

    #[error("Missing value in the config store for key: {0}")]
    MissingConfigVar(String),
}
