extern crate chrono;

use rocket::{post, State, serde::json::Json};
use crate::api::get_current_time::get_current_time;
use crate::models::{Database, Item, LevelCount, AnswerStat};
use super::calculate_progress::calculate_progress;

#[post("/level-count", format = "json", data = "<answer_stats>")]
pub fn level_count(dbs: &State<Database>, answer_stats: Json<Vec<AnswerStat>>) -> Json<Vec<LevelCount>> {
    let db = &dbs.word_db;

    // Use a HashMap to store counts and answer stats for each level
    let mut level_counts = std::collections::HashMap::new();

    // Initialize level counts
    for item in db.iter().filter_map(|item| item.ok()) {
        if let Ok(item) = bincode::deserialize::<Item>(&item.1) {
            let entry = level_counts.entry(item.level).or_insert_with(|| LevelCount {
                level: item.level,
                count: 0,
                total_correct: 0,
                total_incorrect: 0,
                progress: 0.0,
                total_score: 0.0,
            });
            entry.count += 1; // Count the number of items (items) per level
        }
    }

    // Aggregate correct and incorrect answers for each item
    for stat in answer_stats.iter() {
        
        if let Some(item) = db.iter().find(|item| item.as_ref().ok().map_or(false, |(key, _)| key == stat.id.as_bytes())) {
            if let Ok(item) = bincode::deserialize::<Item>(&item.unwrap().1) {
                if let Some(entry) = level_counts.get_mut(&item.level) {
                    entry.total_correct += stat.g;
                    entry.total_incorrect += stat.w;
                    
                    // let score = weighted_score(&stat, get_current_time());
                    let score = calculate_progress(&stat, get_current_time());
                    entry.total_score += score;
                    entry.total_score += score; // Add a new field total_score to LevelCount
                }
            }
        }
    }

    // Calculate progress for each level
    for (_, level_count) in level_counts.iter_mut() {
        if level_count.count > 0 {
            // Normalize the score. This depends on your scoring system.
            // For instance, if each question is worth 1 point, you could do:
            let max_score = level_count.count as f32;
            level_count.progress = (level_count.total_score / max_score).max(0.0).min(1.0) * 100.0;
        }
    }

    let counts = level_counts.into_iter()
        .map(|(_, count)| count)
        .collect::<Vec<_>>();

    Json(counts)
}