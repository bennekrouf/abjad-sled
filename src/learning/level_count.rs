use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelCount {
    pub level: i32,
    pub count: usize,
    pub total_correct: i32,
    pub total_incorrect: i32,
    pub progress: f32,
    pub total_score: f32,
}