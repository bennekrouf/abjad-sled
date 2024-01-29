use rocket::post;
use rocket::{serde::json::Json, State};

use crate::learning::models::{
    user_stat::UserStat,
    knowledge_progress::Analytic,
    learning_config::LearningConfig
};
use crate::learning::calculate_item_progress::calculate_item_progress;

#[post("/analytics", format = "json", data = "<user_stats>")]
pub fn analytics(config: &State<LearningConfig>, user_stats: Json<Vec<UserStat>>) 
-> Json<Vec<Analytic>> {
    let item_progress = calculate_item_progress(&**config, &user_stats);

    Json(item_progress)
}
