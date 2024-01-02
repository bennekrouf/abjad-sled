use std::collections::HashMap;
// use log::info;
use rocket::{post, State, serde::json::Json};
// use log::{info, error};
use crate::models::{Database, Item, AnswerStat, AppConfig};
use crate::api::find_lowest_unfinished_level::find_lowest_unfinished_level;
use super::{get_current_time::get_current_time, calculate_progress::calculate_progress};
pub struct CORS;

const SOME_THRESHOLD:f32 = 100.0;

#[post("/content", format = "json", data = "<answer_stats>")]
pub fn content(dbs: &State<Database>, _config: &State<AppConfig>, answer_stats: Json<Vec<AnswerStat>>) -> Json<Vec<Item>> {
    let db = &dbs.word_db;
    let lowest_unfinished_level = find_lowest_unfinished_level(dbs, &answer_stats);

    // info!("lowest_unfinished_level : {}", lowest_unfinished_level.unwrap().clone());
    let stat_map: HashMap<String, &AnswerStat> = answer_stats.iter()
        .map(|stat| (stat.id.clone(), stat))
        .collect();

    let items = db.iter()
        .filter_map(|item| item.ok())
        .filter_map(|(_, value)| bincode::deserialize::<Item>(&value).ok())
        .filter(|item| item.audio.is_some() && Some(item.level) == lowest_unfinished_level)
        .map(|item| {
            let progress = stat_map.get(&item.id)
                .map(|stat| calculate_progress(stat, get_current_time()))
                .unwrap_or(0.0);
            (item, progress)
        })
        .filter(|(_, progress)| *progress < SOME_THRESHOLD)
        .map(|(item, _)| item)
        .collect::<Vec<_>>();

    Json(items)
}
