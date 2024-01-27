use rocket::{get, State, serde::json::Json};
use crate::domain::process_knowledge_progress::process_knowledge_progress;
use crate::models::Database;
use crate::learning::models::{
    user_stat::UserStat,
    learning_config::LearningConfig,
    knowledge_progress::KnowledgeProgress,
};

#[get("/content?<level>", format = "json", data = "<user_stats>")]
pub fn content_by_level(
    dbs: &State<Database>, 
    config: &State<LearningConfig>, 
    user_stats: Json<Vec<UserStat>>, 
    level: Option<i32>
) -> Json<Vec<KnowledgeProgress>> {
    let db = &dbs.word_db;
    let knowledges = process_knowledge_progress(db, &user_stats.into_inner(), &config, level);
    Json(knowledges)
}
