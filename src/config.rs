use serde::Deserialize;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub site_name: String,
    pub root_domain: String,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}