use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Failed to load `.env` file")]
    DotEnvError(#[from] dotenv::Error),

    #[error("Failed to load environment variable: {0}")]
    EnvVarError(String),
}
