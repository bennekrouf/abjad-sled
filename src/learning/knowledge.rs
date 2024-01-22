use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Knowledge {
    pub id: String,
    pub name: Option<String>,
    pub audio: Option<String>,
    pub level: i32,
}