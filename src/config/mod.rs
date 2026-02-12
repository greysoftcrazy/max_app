use figment::{Figment, providers::{Env, Format, Yaml}};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub max_bot_token: String,
    pub file_storage_path: String,
}

pub fn load() -> Result<Config, figment::Error> {
    Figment::from(Yaml::file("Config.yaml"))
        .merge(Env::prefixed("MAX_APP_"))
        .extract()
}