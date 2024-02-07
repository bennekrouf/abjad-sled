use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Analytic {
    pub id: String,
    pub progress: f32,
    pub category: Option<String>,
}