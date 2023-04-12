use strum_macros::EnumIter;

// All different config options
#[derive(Hash, Eq, PartialEq, Debug, Clone, EnumIter)]
pub enum ConfigOptions {
    SessionExpireTime,
    SpecialRoute,
    CookieName,
    StaticDir,
    DbFile,
    Port,
    Ip,
}

// Convert the config options to a string
impl ToString for ConfigOptions {
    fn to_string(&self) -> String {
        match self {
            ConfigOptions::SessionExpireTime => "SESSION_EXPIRE_TIME".to_string(),
            ConfigOptions::SpecialRoute => "SPECIAL_ROUTE".to_string(),
            ConfigOptions::CookieName => "COOKIE_NAME".to_string(),
            ConfigOptions::StaticDir => "STATIC_DIR".to_string(),
            ConfigOptions::DbFile => "DB_FILE".to_string(),
            ConfigOptions::Port => "PORT".to_string(),
            ConfigOptions::Ip => "IP".to_string(),
        }
    }
}
