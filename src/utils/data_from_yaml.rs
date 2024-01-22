use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::error::Error;
use serde::de::DeserializeOwned; // Import DeserializeOwned trait
use serde_yaml;

use crate::utils::yml_path::get_data_folder_path;
use crate::models::AppConfig;

// Define a generic function that works with any type that implements DeserializeOwned
pub fn data_from_yaml<T>(config: &AppConfig) -> Result<(Vec<T>, String), Box<dyn std::error::Error>>
where
    T: DeserializeOwned + Clone, // Specify the required traits
{
    let data_yaml_path = get_data_folder_path();

    match traverse_directory::<T>(&data_yaml_path, config) {
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

// Define a generic function to traverse the directory and process the YAML files
fn traverse_directory<T>(folder_path: &Path, config: &AppConfig) -> Result<(Vec<T>, PathBuf), Box<dyn Error>>
where
    T: DeserializeOwned + Clone, // Specify the required traits
{
    let mut items: Vec<T> = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let (mut sub_items, _sub_dir_path_) = traverse_directory::<T>(&path, config)?;
            items.append(&mut sub_items);
        } else if let Some(extension) = path.extension() {
            if extension == "yml" {
                let mut file = File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                match serde_yaml::from_str::<Vec<T>>(&contents) {
                    Ok(mut data_from_file) => {
                        // Process audio here if needed for the specific type
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
