use rocket::{post, State, serde::json::Json};
use crate::models::Database;
use crate::learning::models::{
    user_stat::UserStat,
    synthesis::LevelAnalytics,
    learning_config::LearningConfig
};
use crate::domain::calculate_level_counts::calculate_level_counts;

#[post("/synthesis", format = "json", data = "<user_stats>")]
pub fn synthesis(config: &State<LearningConfig>, dbs: &State<Database>, user_stats: Json<Vec<UserStat>>) -> Json<Vec<LevelAnalytics>> {
    let mut level_counts = calculate_level_counts(&**config, dbs, &user_stats);

    // Calculate progress for each level
    for synthesis in &mut level_counts {
        let max_score = synthesis.count as f32;
        synthesis.progress = (synthesis.total_score / max_score).max(0.0).min(1.0) * 100.0;
    }

    Json(level_counts)
}