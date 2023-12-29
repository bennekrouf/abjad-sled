use std::path::PathBuf;

use crate::models::{Database, AppConfig};
use crate::domain::word_init;

pub fn init(absolute_data_folder_path: &PathBuf, config: &AppConfig) -> Database {
    // Construct the absolute paths to the database files
    let word_db_path = absolute_data_folder_path.join("word_db");
    let verse_db_path = absolute_data_folder_path.join("verse_db");

    // Open the Sled databases using the adjusted file paths
    let word_db = sled::open(word_db_path).expect("Failed to open words database");
    let verse_db = sled::open(verse_db_path).expect("Failed to open verse database");

    let database = Database {
        word_db,
        verse_db, // used in read-only
    };

    word_init::init(&database, config);

    // Count the number of key/value pairs in each database and print
    let word_db_size = database.word_db.iter().count();
    println!("word_db contains {} key/value pairs", word_db_size);

    database
}
