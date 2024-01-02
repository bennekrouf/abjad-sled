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

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub macos_path: String,
    pub debian_path: String,
    pub port: u16,
    pub domain: String,
    pub static_files_path: String,
    pub decay_rate_factor_correct: f32,
    pub decay_rate_factor_incorrect: f32,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ExerciseType {
    FindSound,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
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

#[derive(Debug, Serialize)]
pub struct LevelCount {
    pub level: i32,
    pub count: usize,
    pub total_correct: i32,
    pub total_incorrect: i32,
    pub progress: f32,
    pub total_score: f32,
}