use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Knowledge {
    pub id: String,
    pub name: Option<String>,
    pub audio: Option<String>,
    pub level: i32,
}

pub trait Levelable {
    fn level(&self) -> i32;
}

impl Levelable for Knowledge {
    fn level(&self) -> i32 {
        self.level
    }
}