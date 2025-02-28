use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub service_id: String,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

// Parse the config file from the given path
// Returns the config if successful, otherwise returns an error
pub fn parse_config<P: AsRef<Path>>(path: P) -> anyhow::Result<AppConfig> {
    let config_str = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&config_str)?;
    Ok(config)
}

impl Default for AppConfig {
    fn default() -> Self {
        let service_id = "prism-be-id".to_string();
        let server = ServerConfig { port: 8080 };
        Self { service_id, server }
    }
}
