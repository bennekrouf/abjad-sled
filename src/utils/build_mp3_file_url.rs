// use log::info;
use crate::models::AppConfig;
use std::path::{Path, PathBuf};

pub fn build_mp3_file_url(config: &AppConfig, full_mp3_path: &Path) -> Result<String, std::io::Error> {
    // Find the first occurrence of 'data' in the path components and reconstruct the path from there
    let mut components = full_mp3_path.components();
    let mut relative_path = PathBuf::new();
    let mut data_found = false;

    while let Some(component) = components.next() {
        if data_found {
            relative_path.push(component);
        } else if component.as_os_str() == "data" {
            data_found = true;
        }
    }

    if !data_found {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Path does not contain 'data'"));
    }

    let relative_path_str = relative_path.to_string_lossy().to_string();

    // Determine if the domain is a local environment (localhost or 127.0.0.1)
    let is_local_env = config.domain == "localhost" || config.domain == "127.0.0.1";

    // Construct the full URL with or without the port based on the environment
    let full_url = if is_local_env {
        format!("http://{}:{}/{}/{}", config.domain, config.port, config.static_files_path, relative_path_str)
    } else {
        format!("http://{}/{}/{}", config.domain, config.static_files_path, relative_path_str)
    };

    // info!("Constructed MP3 file URL: {}", full_url);

    Ok(full_url)
}