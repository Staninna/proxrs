pub use config_store::ConfigStore;
pub use options::ConfigOptions::{self, *};

const PREFIX: &str = "PROXRS_";

mod config_store;
pub mod init;
pub mod options;
