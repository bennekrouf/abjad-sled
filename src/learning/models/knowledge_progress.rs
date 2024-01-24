use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnowledgeProgress {
    pub id: String,
    pub total_correct: i32,
    pub total_incorrect: i32,
    pub progress: f32,
}