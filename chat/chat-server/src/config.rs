use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub db_url: String,
    pub base_url: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub sk: String,
    pub pk: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret = match (
            File::open("chat.yml"),
            File::open("/etc/config/chat.yml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config file not found"),
        };
        Ok(ret?)
    }
}
