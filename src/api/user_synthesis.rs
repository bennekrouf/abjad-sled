use rocket::{post, State, serde::json::Json};
use crate::models::Database;
use crate::learning::models::{
    user_stat::UserStat,
    level_analytics::LevelAnalytics,
    learning_config::LearningConfig
};
use crate::domain::user_level_analytics::user_level_analytics;

#[post("/user-synthesis", format = "json", data = "<user_stats>")]
pub fn user_synthesis(config: &State<LearningConfig>, dbs: &State<Database>, user_stats: Json<Vec<UserStat>>) -> Json<Vec<LevelAnalytics>> {
    let level_counts = user_level_analytics(&**config, dbs, &user_stats);

    // // Calculate progress for each level
    // for synthesis in &mut level_counts {
    //     let max_score = synthesis.count as f32;
    //     synthesis.progress = ((synthesis.total_correct + synthesis.total_incorrect) / max_score).max(0.0).min(1.0) * 100.0;
    // }

    Json(level_counts)
}