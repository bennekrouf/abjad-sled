use rocket::{post, State, serde::json::Json};
use crate::domain::user_analytics::user_analytics;
use crate::domain::user_level_analytics::user_level_analytics;
use crate::models::Database;
use crate::learning::models::{
    user_stat::UserStat,
    learning_config::LearningConfig,
    analytic::Analytic,
};

#[post("/user-content?<level>", format = "json", data = "<user_stats>")]
pub fn user_content(
    dbs: &State<Database>, 
    config: &State<LearningConfig>, 
    user_stats: Json<Vec<UserStat>>, 
    level: Option<u8>
) -> Json<Vec<Analytic>> {
    let user_stats_inner = user_stats.into_inner(); // Move occurs here, but it's okay since we don't use `user_stats` again.

    // Now, pass references to `user_stats_inner` to both functions.
    let level_analytics = user_level_analytics(&config, dbs, &user_stats_inner);

    let should_fetch_knowledge = if let Some(lvl) = level {
        level_analytics.iter().any(|la| la.level == lvl && la.progress < 100.0)
    } else {
        true // If no specific level is specified, or to fetch all knowledge.
    };

    let db = &dbs.word_db;
    let knowledges = if should_fetch_knowledge {
        user_analytics(db, &user_stats_inner, &config, level)
    } else {
        vec![] // Return an empty vector if the condition is not met.
    };

    Json(knowledges)
}
