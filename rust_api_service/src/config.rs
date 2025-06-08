use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 4567,
            },
            database: DatabaseConfig {
                url: "sqlite::memory:".to_string(),
            },
        }
    }
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let _run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
    
    let s = Config::builder()
        // Start with default values
        .add_source(Config::try_from(&AppConfig::default())?)
        // Add in settings from the environment (with a prefix of APP)
        // E.g., `APP_SERVER__PORT=4567` would set `AppConfig.server.port`
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?;

    s.try_deserialize()
}
