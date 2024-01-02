use std::path::PathBuf;
use serde_yaml;
use std::env;
use crate::models::AppConfig;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref CONFIG: Mutex<AppConfig> = Mutex::new(load_app_config());
}

fn load_app_config() -> AppConfig {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());
    load_config(&app_env)
}

pub fn load_config(app_env: &str) -> AppConfig {
    let config_path = format!("config.{}.yml", app_env);
    let config_str = std::fs::read_to_string(&config_path)
        .expect("Failed to read config file");
    serde_yaml::from_str(&config_str).expect("Failed to parse config file")
}

pub fn get_data_folder_path() -> PathBuf {
    let mut path = PathBuf::new();

    // Access the lazily loaded configuration
    let config = CONFIG.lock().unwrap();

    // Push the appropriate path based on OS target
    if cfg!(target_os = "macos") {
        path.push(&config.macos_path);
    } else {
        path.push(&config.debian_path);
    }

    path
}