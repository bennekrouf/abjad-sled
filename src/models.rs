use sled::Db;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Database {
    pub word_db: Db,
    pub verse_db: Db,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            word_db: self.word_db.clone(),
            verse_db: self.verse_db.clone(),
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Letter {
    pub id: String,
    pub name: Option<String>,
    pub audio: Option<String>,
    pub level: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnswerStat {
    pub id: String,
    pub g: i32,
    pub w: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}
