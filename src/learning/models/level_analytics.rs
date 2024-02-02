use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LevelAnalytics {
    pub level: u8,
    pub count: usize,
    pub progress: f32,
}