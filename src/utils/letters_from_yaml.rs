use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::error::Error;
// use log::info;
use serde_yaml;
// use log::info;

use crate::utils::{yml_path::get_data_folder_path, build_mp3_file_url::build_mp3_file_url};
use crate::utils::build_mp3_file_path::build_mp3_file_path;
use crate::models::{Letter, AppConfig};

pub fn letters_from_yaml(config: &AppConfig) -> Result<(Vec<Letter>, String), Box<dyn std::error::Error>> {
    let letters_yaml_path = get_data_folder_path();

    match traverse_directory(&letters_yaml_path, config) {
        Ok((letters, yaml_path)) => {
            let yaml_path_str = yaml_path.to_string_lossy().into_owned();
            Ok((letters, yaml_path_str))
        },
        Err(e) => {
            eprintln!("Error loading letters: {}", e);
            Err(e)
        }
    }
}

// Function to traverse the directory and process the YAML files.
fn traverse_directory(folder_path: &Path, config: &AppConfig) -> Result<(Vec<Letter>, PathBuf), Box<dyn Error>> {
    let mut letters: Vec<Letter> = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let (mut sub_letters, _sub_dir_path_) = traverse_directory(&path, config)?;
            letters.append(&mut sub_letters);
        } else if let Some(extension) = path.extension() {
            if extension == "yml" {
                let mut file = File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                match serde_yaml::from_str::<Vec<Letter>>(&contents) {
                    Ok(mut data_from_file) => {
                        for letter in &mut data_from_file {
                            if let Some(audio) = &letter.audio {
                                // info!("letter.audio before: {:?}", &letter.audio);
                                let audio_path = build_mp3_file_path(&path, audio);
                                // info!("audio_path before build_mp3_file_url: {:?}", audio_path);
                                match build_mp3_file_url(&config, &audio_path) {
                                    Ok(url) => letter.audio = Some(url),
                                    Err(e) => eprintln!("Error building mp3 file URL: {}", e),
                                }
                            }
                        }
                        
                        letters.append(&mut data_from_file);
                    },
                    Err(e) => {
                        return Err(format!("Failed to deserialize file at {:?}: {}", path, e).into());
                    }
                }
            }
        }
    }
    
    Ok((letters, PathBuf::from(&config.debian_path)))
}