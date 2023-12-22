use sled::Db;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Database {
    pub words_db: Db,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            words_db: self.words_db.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub macos_path: String,
    pub debian_path: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ExerciseType {
    FindSound,
}