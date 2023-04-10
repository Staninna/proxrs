pub use config_store::Config;
pub use options::ConfigOptions::{self, *};

const PREFIX: &str = "PROXRS_";

mod config_store;
pub mod init;
pub mod options;
