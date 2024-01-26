use rocket::post;
use rocket::{serde::json::Json, State};

use crate::learning::models::{user_stat::UserStat, knowledge_progress::KnowledgeProgress};
use crate::learning::calculate_item_progress::calculate_item_progress;
use crate::learning::models::learning_config::LearningConfig;

#[post("/level-count-detail", format = "json", data = "<user_stats>")]
pub fn level_count_detail(config: &State<LearningConfig>, user_stats: Json<Vec<UserStat>>) -> Json<Vec<KnowledgeProgress>> {
    let config = &**config;
    let item_progress = calculate_item_progress(config, &user_stats);

    Json(item_progress)
}
