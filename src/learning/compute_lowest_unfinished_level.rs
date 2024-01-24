use std::collections::HashMap;
use serde::Deserialize;
use crate::learning::{user_stat::UserStat, knowledge::Levelable};
use super::calculate_progress::calculate_progress;

const SOME_THRESHOLD:f32 = 100.0;

// Function to perform the computation
pub fn compute_lowest_unfinished_level<'de, T>(entries: &HashMap<String, T>, user_stats: &[UserStat]) -> Option<i32>
    where T: PartialEq + Clone + Deserialize<'de> + Levelable,
{
    // If user_stats is empty, return the first level
    if user_stats.is_empty() {
        // debug!("No answer stats available, returning first level.");
        return Some(1);
    }
    let mut level_aggregate: HashMap<i32, (f32, i32)> = HashMap::new();

    // Iterate over the retrieved entries
    for (key, item) in entries {
        let key_str = key.clone();
        let progress = calculate_progress(user_stats.iter().find(|s| s.id == key_str).unwrap());
        let entry = level_aggregate.entry(item.level()).or_insert((0.0, 0));
        entry.0 += progress;
        entry.1 += 1;
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