// use log::info;
use crate::utils::items_from_yaml::letters_from_yaml;
use crate::utils::insert_in_sled;
use crate::models::{Database, AppConfig};

pub fn init(dbs: &Database, config: &AppConfig) {
    let (items, _yaml_path) = letters_from_yaml(config).expect("Failed to load YAML file");
    for item in items {
        let key = item.id.clone();
        insert_in_sled::insert_in_sled(&dbs.word_db, &key, &item);
    }
}