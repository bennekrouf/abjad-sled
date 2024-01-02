use std::collections::HashMap;
// use log::{debug, info};
use rocket::State;
use crate::models::{AnswerStat, Database, Letter};
use super::{calculate_progress::calculate_progress, get_current_time::get_current_time};

pub fn find_lowest_unfinished_level(dbs: &State<Database>, answer_stats: &[AnswerStat]) -> Option<i32> {
    // If answer_stats is empty, return the first level
    if answer_stats.is_empty() {
        // debug!("No answer stats available, returning first level.");
        return Some(1);
    }
    let mut level_aggregate: HashMap<i32, (f32, i32)> = HashMap::new();
    let db = &dbs.word_db;
    const SOME_THRESHOLD: f32 = 100.0;

    // Aggregate progress for each level
    for item in db.iter().filter_map(|item| item.ok()) {
        if let Ok(letter) = bincode::deserialize::<Letter>(&item.1) {
            // info!("Processing letter: {:?}", letter);
            if let Some(stat) = answer_stats.iter().find(|s| s.id == letter.id) {
                let progress = calculate_progress(stat, get_current_time());
                // info!("Letter ID: {}, Progress: {}", letter.id, progress);
                let entry = level_aggregate.entry(letter.level).or_insert((0.0, 0));
                entry.0 += progress; // Total progress for the level
                entry.1 += 1;        // Count of letters for the level
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
