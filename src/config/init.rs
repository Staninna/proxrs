use crate::{config::*, error::Error};
use dotenv::{dotenv, var};
use strum::IntoEnumIterator;

pub fn conf() -> Result<Config, Error> {
    // Load the .env file
    dotenv()?;

    // Iterate over all config options
    let mut conf = Config::new();
    for key in ConfigOptions::iter() {
        // Get the environment variable
        let value = var(PREFIX.to_owned() + &key.to_string())
            .map_err(|_| Error::MissingEnvVar(key.to_string()))?;

        // Check if the value is empty
        if value.is_empty() {
            return Err(Error::EmptyEnvVar(key.to_string()));
        }

        // Set the value in the config store
        conf.set(key, value);
    }

    // Return the config store
    Ok(conf)
}
