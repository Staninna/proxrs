use super::{error::ConfigError, options::ConfigOptions, store::Config, PREFIX};
use dotenv::{dotenv, var};
use strum::IntoEnumIterator;

pub fn conf() -> Result<Config, ConfigError> {
    // Load the .env file
    dotenv().map_err(|_| ConfigError::DotEnvError)?;

    // Iterate over all config options
    let mut conf = Config::new();
    for key in ConfigOptions::iter() {
        // Get the environment variable
        let value = var(PREFIX.to_owned() + &key.to_string())
            .map_err(|_| ConfigError::EnvVarError(key.to_string()))?;

        // Set the value in the config store
        conf.set(key, value);
    }

    // Return the config store
    Ok(conf)
}
