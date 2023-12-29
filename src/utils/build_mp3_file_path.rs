// use std::env;
use std::path::{Path, PathBuf};
// use log::info;

// use crate::models::AppConfig;

pub fn build_mp3_file_path(yaml_file_path: &Path, audio_filename: &str) -> PathBuf {
    let current_folder = yaml_file_path.parent().unwrap_or_else(|| Path::new(""));
    current_folder.join(audio_filename)
}