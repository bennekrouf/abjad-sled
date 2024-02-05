use serde_yaml;
use std::path::PathBuf;
use std::sync::Mutex;
use std::env;
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;

use crate::models::AppConfig;
use crate::learning::models::learning_config::LearningConfig;

pub trait Config {
    fn load(config_path: &str) -> Self;
}

lazy_static! {
    pub static ref CONFIG: Mutex<AppConfig> = Mutex::new(AppConfig::load("config"));
    pub static ref LEARNING: Mutex<LearningConfig> = Mutex::new(LearningConfig::load("learning"));
}

impl Config for AppConfig {
    fn load(config_path: &str) -> Self {
        load_config(config_path)
    }
}

impl Config for LearningConfig {
    fn load(config_path: &str) -> Self {
        load_config(config_path)
    }
}

pub fn load_config<T>(config_path: &str) -> T
    where T: DeserializeOwned,
{
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());
    let config_path = format!("{}.{}.yml", config_path, app_env);
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