pub use config_store::ConfigStore;
use error::Error;
pub use options::ConfigOptions::{self, *};

const PREFIX: &str = "PROXRS_";

mod config_store;
pub mod error;
pub mod init;
pub mod options;
