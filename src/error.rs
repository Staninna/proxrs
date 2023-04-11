use thiserror::Error;

// Global error type (inherits from all other errors)
#[derive(Error, Debug)]
pub enum Error {
    // Missing config variable
    #[error("Missing config variable: {0}")]
    MissingConfigVar(String),

    // Missing environment variable
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    // Empty environment variable
    #[error("Empty environment variable: {0}")]
    EmptyEnvVar(String),

    // Dotenv error
    #[error("Dotenv: {0}")]
    Dotenv(#[from] dotenv::Error),

    // Database error
    #[error("Database: {0}")]
    Database(#[from] rusqlite::Error),
}

#[macro_export]
macro_rules! check_err {
    ($e:expr) => {
        $e.unwrap_or_else(|err| {
            // Get line/file/charecter
            let (line, file, charecter) = (line!(), file!(), column!());

            // Print the error
            eprintln!("{}:{}:{}: {}", file, line, charecter, err);

            // Exit the program
            std::process::exit(1);
        })
    };
}
