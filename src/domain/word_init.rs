use crate::utils::data_from_yaml::data_from_yaml;
use crate::learning::knowledge::Knowledge;
use crate::models::{Database, AppConfig};
use crate::utils::insert_in_sled;
use crate::utils::{
    yml_path::get_data_folder_path,
    build_mp3_file_path::build_mp3_file_path,
    build_mp3_file_url::build_mp3_file_url
};

pub fn init(dbs: &Database, config: &AppConfig) {
    let (mut items, _yaml_path) = data_from_yaml::<Knowledge>(config).expect("Failed to load YAML file");
    let knowledge_yaml_path = get_data_folder_path();
    for item in &mut items {
        if let Some(audio) = &item.audio {
            let audio_path = build_mp3_file_path(&knowledge_yaml_path, audio);
            match build_mp3_file_url(&config, &audio_path) {
                Ok(url) => item.audio = Some(url),
                Err(e) => eprintln!("Error building mp3 file URL: {}", e),
            }
        }
    }

    for item in items {
        let key = item.id.clone();
        insert_in_sled::insert_in_sled(&dbs.word_db, &key, &item);
    }
}