use rocket::{get, State, serde::json::Json};
use std::collections::HashMap;
use crate::models::Database;
use crate::learning::models::knowledge::Knowledge;
use crate::domain::knowledge_entries::knowledge_entries;

#[get("/knowledge-entries?<level>", format = "json")]
pub fn get_knowledge_entries(
    dbs: &State<Database>,
    level: Option<u8>
) -> Json<HashMap<String, Knowledge>> {
    let db = &dbs.word_db;
    let knowledge_map = knowledge_entries(db, level);
    Json(knowledge_map)
}
