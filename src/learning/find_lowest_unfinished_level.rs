use std::collections::HashMap;
// use log::{debug, info};
use rocket::State;
use crate::models::Database;
use crate::learning::knowledge::Knowledge;
use crate::learning::user_stat::UserStat;

use super::calculate_progress::calculate_progress;

pub fn find_lowest_unfinished_level(dbs: &State<Database>, user_stats: &[UserStat]) -> Option<i32> {
    // If user_stats is empty, return the first level
    if user_stats.is_empty() {
        // debug!("No answer stats available, returning first level.");
        return Some(1);
    }
    let mut level_aggregate: HashMap<i32, (f32, i32)> = HashMap::new();
    let db = &dbs.word_db;
    const SOME_THRESHOLD: f32 = 100.0;

    // Aggregate progress for each level
    for item in db.iter().filter_map(|item| item.ok()) {
        if let Ok(item) = bincode::deserialize::<Knowledge>(&item.1) {
            // info!("Processing item: {:?}", item);
            if let Some(stat) = user_stats.iter().find(|s| s.id == item.id) {
                let progress = calculate_progress(stat);
                // info!("Knowledge ID: {}, Progress: {}", item.id, progress);
                let entry = level_aggregate.entry(item.level).or_insert((0.0, 0));
                entry.0 += progress; // Total progress for the level
                entry.1 += 1;        // Count of items for the level
            }
        }
    }

    // Calculate average progress for each level
    let level_progress: HashMap<i32, f32> = level_aggregate
        .iter()
        .map(|(&level, &(total_progress, count))| {
             let avg_progress = if count > 0 { total_progress / count as f32 } else { 0.0 };
            //  info!("Level: {}, Average Progress: {}", level, avg_progress);
             (level, avg_progress)
        })
        .collect();

    let lowest_level = level_progress
        .iter()
        .filter(|&(_, &progress)| progress < SOME_THRESHOLD)
        .min_by_key(|&(level, _)| level)
        .map(|(level, _)| *level);

    // info!("Lowest unfinished level: {:?}", lowest_level);

    lowest_level
}
