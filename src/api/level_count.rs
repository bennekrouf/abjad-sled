use rocket::{post, State, serde::json::Json};
use crate::models::Database;
use crate::learning::models::{user_stat::UserStat, level_count::LevelCount};
use super::calculate_level_counts::calculate_level_counts;

#[post("/level-count", format = "json", data = "<user_stats>")]
pub fn level_count(dbs: &State<Database>, user_stats: Json<Vec<UserStat>>) -> Json<Vec<LevelCount>> {
    let mut level_counts = calculate_level_counts(dbs, &user_stats);

    // Calculate progress for each level
    for level_count in &mut level_counts {
        let max_score = level_count.count as f32;
        level_count.progress = (level_count.total_score / max_score).max(0.0).min(1.0) * 100.0;
    }

    Json(level_counts)
}