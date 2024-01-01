// use log::info;
use crate::utils::letters_from_yaml::letters_from_yaml;
use crate::utils::insert_in_sled;
use crate::models::{Database, AppConfig};

pub fn init(dbs: &Database, config: &AppConfig) {
    let (letters, _yaml_path) = letters_from_yaml(config).expect("Failed to load YAML file");
    for letter in letters {
        let key = letter.id.clone();
        insert_in_sled::insert_in_sled(&dbs.word_db, &key, &letter);
    }
}