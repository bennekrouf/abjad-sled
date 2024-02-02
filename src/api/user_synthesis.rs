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

    Json(level_counts)
}