// use crate::learning::knowledge::Knowledge;
use crate::models::AppConfig;
use crate::learning::models::knowledge::HasAudio;
use serde::de::DeserializeOwned; 

use crate::utils::{
    yml_path,
    traverse_directory::traverse_directory,
};

pub fn data_from_yaml<'de, T>(config: &AppConfig)
    -> Result<(Vec<T>, String), Box<dyn std::error::Error>>
    where
        T: Clone + DeserializeOwned + std::fmt::Debug + HasAudio,
{
    let data_yaml_path = yml_path::get_data_folder_path();

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