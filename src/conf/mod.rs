pub use config::Config;
pub use options::ConfigOptions::{self, *};

const PREFIX: &str = "PROXRS_";

mod config;
pub mod init;
pub mod options;
