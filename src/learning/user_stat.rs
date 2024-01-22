use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserStat {
    pub id: String,
    pub g: i32,
    pub w: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}
