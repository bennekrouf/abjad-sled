use rocket::{get, State, serde::json::Json};
use crate::domain::user_analytics::user_analytics;
use crate::models::Database;
use crate::learning::models::{
    user_stat::UserStat,
    learning_config::LearningConfig,
    analytic::Analytic,
};

#[get("/user-content?<level>", format = "json", data = "<user_stats>")]
pub fn user_content(
    dbs: &State<Database>, 
    config: &State<LearningConfig>, 
    user_stats: Json<Vec<UserStat>>, 
    level: Option<i32>
) -> Json<Vec<Analytic>> {
    let db = &dbs.word_db;
    let knowledges = user_analytics(db, &user_stats.into_inner(), &config, level);
    Json(knowledges)
}
