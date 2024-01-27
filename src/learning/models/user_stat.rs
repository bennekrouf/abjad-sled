use serde::{Deserialize, Serialize};

use super::knowledge::Knowledge;

#[derive(Debug, Deserialize, Serialize)]

pub struct UserStat {
    pub id: Option<String>,
    pub knowledge: Option<Knowledge>,
    pub g: i32,
    pub w: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    pub repetitions: Option<i32>,
}
