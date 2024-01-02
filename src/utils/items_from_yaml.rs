use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::error::Error;
// use log::info;
use serde_yaml;

use crate::utils::{yml_path::get_data_folder_path, build_mp3_file_url::build_mp3_file_url};
use crate::utils::build_mp3_file_path::build_mp3_file_path;
use crate::models::{Item, AppConfig};

pub fn letters_from_yaml(config: &AppConfig) -> Result<(Vec<Item>, String), Box<dyn std::error::Error>> {
    let letters_yaml_path = get_data_folder_path();

    match traverse_directory(&letters_yaml_path, config) {
        Ok((items, yaml_path)) => {
            let yaml_path_str = yaml_path.to_string_lossy().into_owned();
            Ok((items, yaml_path_str))
        },
        Err(e) => {
            eprintln!("Error loading items: {}", e);
            Err(e)
        }
    }
}

// Function to traverse the directory and process the YAML files.
fn traverse_directory(folder_path: &Path, config: &AppConfig) -> Result<(Vec<Item>, PathBuf), Box<dyn Error>> {
    let mut items: Vec<Item> = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let (mut sub_letters, _sub_dir_path_) = traverse_directory(&path, config)?;
            items.append(&mut sub_letters);
        } else if let Some(extension) = path.extension() {
            if extension == "yml" {
                let mut file = File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                match serde_yaml::from_str::<Vec<Item>>(&contents) {
                    Ok(mut data_from_file) => {
                        for item in &mut data_from_file {
                            if let Some(audio) = &item.audio {
                                // info!("item.audio before: {:?}", &item.audio);
                                let audio_path = build_mp3_file_path(&path, audio);
                                // info!("audio_path before build_mp3_file_url: {:?}", audio_path);
                                match build_mp3_file_url(&config, &audio_path) {
                                    Ok(url) => item.audio = Some(url),
                                    Err(e) => eprintln!("Error building mp3 file URL: {}", e),
                                }
                            }
                        }
                        
                        items.append(&mut data_from_file);
                    },
                    Err(e) => {
                        return Err(format!("Failed to deserialize file at {:?}: {}", path, e).into());
                    }
                }
            }
        }
    }
    
    Ok((items, PathBuf::from(&config.debian_path)))
}