use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LevelAnalytics {
    pub level: u32,
    pub count: usize,
    pub progress: f32,
}