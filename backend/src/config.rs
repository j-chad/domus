use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
    pub max_pool_size: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub auth: Auth,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        //config file is project_root/config.toml
        let s = Config::builder()
            .add_source(File::with_name("config"))
            .add_source(Environment::with_prefix("domus"))
            .build()?;

        s.try_deserialize()
    }
}
