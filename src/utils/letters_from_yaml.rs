use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::error::Error;
use serde_yaml;

use crate::utils::yml_path;
use crate::models::Letter;

pub fn load() -> Result<(Vec<Letter>, String), Box<dyn std::error::Error>> {
    let data_folder_path = yml_path::get_data_folder_path();
    let letters_yaml_path = data_folder_path.join("letters");
    println!("letters_yaml_path : {:?}", letters_yaml_path);

    match traverse_directory(&letters_yaml_path) {
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
fn traverse_directory(folder_path: &Path) -> Result<(Vec<Letter>, PathBuf), Box<dyn Error>> {
    let mut letters: Vec<Letter> = Vec::new();
    let mut dir_path = PathBuf::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let (mut sub_letters, sub_dir_path) = traverse_directory(&path)?;
            letters.append(&mut sub_letters);
            if !sub_dir_path.as_os_str().is_empty() {
                dir_path = sub_dir_path;
            }
        } else if let Some(extension) = path.extension() {
            if extension == "yml" {
                dir_path = path.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
                let mut file = File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                match serde_yaml::from_str::<Vec<Letter>>(&contents) {
                    Ok(mut data_from_file) => {
                        // Prepend the directory path to the `audio` field of each `Letter`
                        for letter in &mut data_from_file {
                            if let Some(audio) = &letter.audio {
                                // Convert the `PathBuf` to a string representation using `display`
                                let audio_path = format!("{}/{}", dir_path.display(), audio);
                                letter.audio = Some(audio_path);
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
    
    Ok((letters, dir_path))
}