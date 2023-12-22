use std::path::PathBuf;

use crate::models::Database;
use crate::domain::words_init;

pub fn init(absolute_data_folder_path: &PathBuf) -> Database {
    // Construct the absolute paths to the database files
    let words_db_path = absolute_data_folder_path.join("words_db");

    // Open the Sled databases using the adjusted file paths
    let words_db = sled::open(words_db_path).expect("Failed to open words database");

    let database = Database {
        words_db,
    };

    words_init::init(&database);

    // Count the number of key/value pairs in each database and print
    let words_db_size = database.words_db.iter().count();
    println!("words_db contains {} key/value pairs", words_db_size);

    database
}
