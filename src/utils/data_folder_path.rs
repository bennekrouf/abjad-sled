use std::path::{Path, PathBuf};
use std::env;
use users::get_current_username;
use dirs::home_dir;

pub fn get() -> PathBuf {
    let username = get_current_username()
        .expect("Failed to retrieve current username")
        .into_string()
        .expect("Failed to convert username to string");

    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

    if let Some(mut home_path) = home_dir() {
        home_path.push(&app_env);
        home_path.push("wordsDB");
        return home_path;
    }

    Path::new("/tmp").join(&username).join(&app_env).join("wordsDB")
}