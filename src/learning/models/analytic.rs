use serde::{Deserialize, Serialize};

use super::knowledge::Knowledge;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Analytic {
    pub id: String,
    pub progress: f32,
}