use strum_macros::EnumIter;

// All different config options
#[derive(Hash, Eq, PartialEq, Debug, Clone, EnumIter)]
pub enum ConfigOptions {
    Address,
    DbFile,
    Port,
}

// Convert the config options to a string
impl ToString for ConfigOptions {
    fn to_string(&self) -> String {
        match self {
            ConfigOptions::Address => "ADDRESS".to_string(),
            ConfigOptions::DbFile => "DB_FILE".to_string(),
            ConfigOptions::Port => "PORT".to_string(),
        }
    }
}
