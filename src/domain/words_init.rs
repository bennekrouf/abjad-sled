// use crate::files::chapters_from_yaml::load;
// use crate::utils::insert_in_sled;

use crate::models::Database;
use crate::utils::yml_path;

pub fn init(_dbs: &Database) {
    let _data_folder_path = yml_path::get_data_folder_path();
    // let chapters = load(data_folder_path.join("chapters.yaml").to_str().unwrap());
    // for chapter in chapters {
    //     let key = chapter.no.to_be_bytes().to_vec();
    //     insert_in_sled::insert_in_sled(&dbs.chapter_db, key, &chapter);
    // }
}