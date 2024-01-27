use rocket::{post, State, serde::json::Json};
use crate::domain::process_knowledge_progress::process_knowledge_progress;
// use log::{info, error};
use crate::models::Database;
use crate::learning::models::{user_stat::UserStat, learning_config::LearningConfig, knowledge_progress::KnowledgeProgress};

#[post("/content-all", format = "json", data = "<user_stats>")]
pub fn content_all(
    dbs: &State<Database>, 
    config: &State<LearningConfig>, 
    user_stats: Json<Vec<UserStat>>
) -> Json<Vec<KnowledgeProgress>> {
    let db = &dbs.word_db;
    let knowledges = process_knowledge_progress(db, &user_stats, &config, None);
    Json(knowledges)
}