use crate::utils::data_from_yaml::data_from_yaml;
use crate::learning::models::knowledge::Knowledge;
use crate::models::{Database, AppConfig};
use crate::utils::insert_in_sled;

pub fn init(dbs: &Database, config: &AppConfig) {
    let (items, _) = data_from_yaml::<Knowledge>(config).expect("Failed to load YAML file");
    for item in items {
        let key = item.id.clone();
        insert_in_sled::insert_in_sled(&dbs.word_db, &key, &item);
    }
}