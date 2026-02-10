use figment::{Figment, providers::{Env, Format, Toml}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub max_bot_token: String,
    pub file_storage_path: String,
}

pub fn load() -> Result<Config, figment::Error> {
    Figment::from(Toml::file("Config.toml"))
        .merge(Env::prefixed("MAX_APP_"))
        .extract()
}