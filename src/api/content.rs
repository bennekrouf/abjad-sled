use std::collections::HashMap;
use rocket::{post, State, serde::json::Json};
// use log::{info, error};
use crate::models::{Database, AppConfig};
use crate::learning::models::knowledge::Knowledge;
use crate::learning::models::user_stat::UserStat;
use crate::utils::find_lowest_unfinished_level::find_lowest_unfinished_level;
use crate::learning::calculate_progress::calculate_progress;

const SOME_THRESHOLD:f32 = 100.0;
const CONSECUTIVE_HOURS_THRESHOLD: i64 = 0;

#[post("/content", format = "json", data = "<user_stats>")]
pub fn content(dbs: &State<Database>, _config: &State<AppConfig>, user_stats: Json<Vec<UserStat>>) -> Json<Vec<Knowledge>> {
    let db = &dbs.word_db;
    let lowest_unfinished_level = find_lowest_unfinished_level::<Knowledge>(db, &user_stats, CONSECUTIVE_HOURS_THRESHOLD);

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
                .map(|stat| calculate_progress(stat, CONSECUTIVE_HOURS_THRESHOLD))
                .unwrap_or(0.0);
            (item, progress)
        })
        .filter(|(_, progress)| *progress < SOME_THRESHOLD)
        .map(|(item, _)| item)
        .collect::<Vec<_>>();

    Json(items)
}
