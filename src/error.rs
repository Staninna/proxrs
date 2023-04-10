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

/// A macro to check for errors and print them. If an expression evaluates to an error,
/// it will print the error message, along with the file name, line number, and character
/// position where the error occurred. The macro will then exit the program with an exit code of 1.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::Read;
///
/// fn main() {
///     // Read the contents of a file
///     let mut file = err!(File::open("/non/existent/file"));
///     unreachable!("This code will never be reached");
/// }
/// ```
///
/// # Notes
///
/// This macro uses the `unwrap_or_else` method to handle errors. If the expression being
/// checked returns an `Ok` value, that value is returned as is. If it returns an `Err`,
/// the closure passed to `unwrap_or_else` will be executed, which prints the error message
/// and exits the program. The purpose of this macro is to provide a more expressive way to
/// handle errors in Rust code, compared to the built-in `?` operator or the `unwrap` method.
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
