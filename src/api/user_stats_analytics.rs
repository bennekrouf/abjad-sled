use rocket::post;
use rocket::{serde::json::Json, State};

use crate::learning::models::{
    user_stat::UserStat,
    analytic::Analytic,
    learning_config::LearningConfig
};
use crate::learning::compute_user_stats_analytics::compute_user_stats_analytics;

#[post("/user-stats-analytics", format = "json", data = "<user_stats>")]
pub fn user_stats_analytics(config: &State<LearningConfig>, user_stats: Json<Vec<UserStat>>) 
-> Json<Vec<Analytic>> {
    let item_progress = compute_user_stats_analytics(&**config, &user_stats);

    Json(item_progress)
}
