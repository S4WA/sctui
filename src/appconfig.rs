use std::collections::HashMap;
use std::fs::{self, create_dir_all};
use std::path::Path;
use config::Config;

#[derive(Debug)]
pub struct AppConfig {
    client_id: String,
}

impl AppConfig {
    pub fn new(client_id: &str) -> Self {
        AppConfig {
            client_id: client_id.to_string(),
        }
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }
}

pub fn init() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = "config/config.toml";
    let config_dir = "config";

    // Check if config file exists, if not create new
    if !Path::new(config_path).exists() {
        // Create directories if they don't exist
        create_dir_all(config_dir)?;

        // Default configuration content
        let default_config = r#"client_id = "YOUR_CLIENT_ID"
"#;

        // Write default config to file
        fs::write(config_path, default_config)?;
    }

    let settings = Config::builder()
        .add_source(config::File::with_name("config/config"))
        .add_source(config::Environment::with_prefix("SCTUI"))
        .build()?;

    let config_map = settings.try_deserialize::<HashMap<String, String>>()?;
    
    // Get client_id from config_map, with a fallback default
    let client_id = config_map
        .get("client_id")
        .ok_or("client_id not found in configuration")?;
    
    let app_config = AppConfig::new(client_id);
    
    // println!("Loaded configuration: {:?}", config_map);
    // println!("Using Client ID: {}", app_config.get_client_id());
    
    Ok(app_config)
}

