use rocket::post;
use rocket::{serde::json::Json};

use crate::learning::models::{user_stat::UserStat, knowledge_progress::KnowledgeProgress};
use crate::learning::calculate_item_progress::calculate_item_progress;
const CONSECUTIVE_HOURS_THRESHOLD: i64 = 0;

#[post("/level-count-detail", format = "json", data = "<user_stats>")]
pub fn level_count_detail(user_stats: Json<Vec<UserStat>>) -> Json<Vec<KnowledgeProgress>> {
    let item_progress = calculate_item_progress(&user_stats, CONSECUTIVE_HOURS_THRESHOLD);

    Json(item_progress)
}
