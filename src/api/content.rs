use std::collections::HashMap;
use rocket::{post, State, serde::json::Json};
// use log::{info, error};
use crate::models::Database;
use crate::learning::models::knowledge::Knowledge;
use crate::learning::models::user_stat::UserStat;
use crate::utils::find_lowest_unfinished_level::find_lowest_unfinished_level;
use crate::learning::calculate_progress::calculate_progress;
use crate::learning::models::learning_config::LearningConfig;

const SOME_THRESHOLD:f32 = 100.0;

#[post("/content", format = "json", data = "<user_stats>")]
pub fn content(dbs: &State<Database>, config: &State<LearningConfig>, user_stats: Json<Vec<UserStat>>) -> Json<Vec<Knowledge>> {
    let db = &dbs.word_db;
    let config = &**config;
    let lowest_unfinished_level = find_lowest_unfinished_level::<Knowledge>(config, db, &user_stats);

    // info!("lowest_unfinished_level : {}", lowest_unfinished_level.unwrap().clone());
    let stat_map: HashMap<String, &UserStat> = user_stats.iter()
        .map(|stat| (stat.id.clone(), stat))
        .collect();

    let items = db.iter()
        .filter_map(|item| item.ok())
        .filter_map(|(_, value)| bincode::deserialize::<Knowledge>(&value).ok())
        .filter(|item| item.audio.is_some() && Some(item.level) == lowest_unfinished_level)
        .map(|item| {
            let progress = stat_map.get(&item.id)
                .map(|stat| calculate_progress(config, stat))
                .unwrap_or(0.0);
            (item, progress)
        })
        .filter(|(_, progress)| *progress < SOME_THRESHOLD)
        .map(|(item, _)| item)
        .collect::<Vec<_>>();

    Json(items)
}
