use std::env;
use std::path::Path;
use log::info;

use crate::models::AppConfig;

pub fn build_mp3_file_url(config: &AppConfig, full_mp3_path: &Path) -> Result<String, std::io::Error> {
    info!("full_mp3_path: {}", full_mp3_path.display());

    // Convert the full path to a string and then find the 'data/letters' part
    let full_path_str = full_mp3_path.to_string_lossy().to_string();
    let relative_path = if let Some(pos) = full_path_str.find("data/letters/") {
        &full_path_str[pos..]
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Path does not contain 'data/letters'"));
    };

    info!("Relative path for MP3 file: {}", relative_path);

    // Construct the full URL using the relative path
    let full_url = format!("http://{}:{}/{}/{}", config.domain, config.port, config.static_files_path, relative_path);

    info!("Constructed MP3 file URL: {}", full_url);

    Ok(full_url)
}
