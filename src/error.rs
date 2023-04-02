use thiserror::Error;

// Global error type (inherits from all other errors)
#[derive(Error, Debug)]
pub enum Error {
    // Config error
    #[error("Config error: {0}")]
    Config(#[from] crate::config::error::Error),

    // Database error
    #[error("Database error: {0}")]
    Database(#[from] crate::db::error::Error),
}

// Macro to check for errors and print them
// TODO: Remove line/file/charecter when stable
#[macro_export]
macro_rules! err {
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
