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
    pub name: Option<String>,
    pub name_arabic: String,
    pub contextual_forms: Option<ContextualForms>,
    pub audio: Option<String>,
    pub level: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContextualForms {
    pub finalp: Option<String>,
    pub isolated: Option<String>,
    pub medial: Option<String>,
    pub initial: Option<String>,
}