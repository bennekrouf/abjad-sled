use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::error::Error;
use serde::de::DeserializeOwned;
use serde_yaml;
use crate::learning::models::knowledge::HasAudio;

use crate::models::AppConfig;
use crate::utils::{
    build_mp3_file_path::build_mp3_file_path,
    build_mp3_file_url::build_mp3_file_url
};

// Define a generic function to traverse the directory and process the YAML files
pub fn traverse_directory<T>(folder_path: &Path, config: &AppConfig) -> Result<(Vec<T>, PathBuf), Box<dyn Error>>
where
    T: DeserializeOwned + Clone + HasAudio,
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
                        for item in &mut data_from_file {
                            if let Some(audio) = &item.audio() {
                                // info!("item.audio before: {:?}", &item.audio);
                                let audio_path = build_mp3_file_path(&path, audio);
                                // info!("audio_path before build_mp3_file_url: {:?}", audio_path);
                                match build_mp3_file_url(&config, &audio_path) {
                                    Ok(url) => item.set_audio(Some(url)),
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
