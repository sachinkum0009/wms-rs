use color_eyre::{Result, eyre::eyre};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub database_url: String,
}

impl Config {
    pub fn load(
        config_file: Option<&str>, 
        api_url_override: Option<String>,
        database_url_override: Option<String>
    ) -> Result<Self> {
        // Try to load from config file if provided
        if let Some(_config_path) = config_file {
            // TODO: Implement config file loading when needed
            // For now, fall back to environment variables
        }
        
        // Get API URL from override or environment
        let api_url = api_url_override
            .or_else(|| env::var("WMS_API_URL").ok())
            .unwrap_or_else(|| "http://localhost:3000".to_string());
        
        // Get Database URL from override or environment
        let database_url = database_url_override
            .or_else(|| env::var("WMS_DATABASE_URL").ok())
            .unwrap_or_else(|| "postgres://localhost/wms".to_string());
        
        Ok(Config {
            api_url,
            database_url,
        })
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.api_url.is_empty() {
            return Err(eyre!("API URL cannot be empty"));
        }
        
        if self.database_url.is_empty() {
            return Err(eyre!("Database URL cannot be empty"));
        }
        
        Ok(())
    }
}