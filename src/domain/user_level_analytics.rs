use crate::learning::models::user_stat::UserStat;
use crate::models::Database;
use crate::learning::models::{
    knowledge::Knowledge,
    level_analytics::LevelAnalytics,
    learning_config::LearningConfig
};
use crate::learning::compute_user_stat_progress::compute_user_stat_progress;

pub fn user_level_analytics(config: &LearningConfig, db: &Database, user_stats: &Vec<UserStat>) -> Vec<LevelAnalytics> {
    let mut level_progress: std::collections::HashMap<u32, (f32, u32)> = std::collections::HashMap::new();

    // Initialize level counts and accumulate progress separately
    let mut level_counts: Vec<LevelAnalytics> = db.word_db.iter()
        .filter_map(|item| item.ok())
        .filter_map(|(_, data)| bincode::deserialize::<Knowledge>(&data).ok())
        .fold(std::collections::HashMap::new(), |mut counts, item| {
            let level = item.level.unwrap();
            let entry = counts.entry(level).or_insert_with(|| LevelAnalytics {
                level,
                count: 0,
                progress: 0.0,
            });
            entry.count += 1;
            let progress_entry = level_progress.entry(level).or_insert((0.0, 0));
            progress_entry.1 += 1; // Increment item count for level
            counts
        })
        .into_iter()
        .map(|(_, synthesis)| synthesis)
        .collect();

    // Compute progress for each user_stat and accumulate
    for (stat, item) in user_stats.iter()
        .flat_map(|stat| {
            db.word_db.iter()
                .find(|item| item.as_ref().ok().map_or(false, |(key, _)| String::from_utf8_lossy(key).as_ref() == stat.id))
                .and_then(|item| bincode::deserialize::<Knowledge>(&item.unwrap().1).ok())
                .map(|item| (stat, item))
        }) {
        let progress = compute_user_stat_progress(config, &stat);
        if let Some(progress_entry) = level_progress.get_mut(&item.level.unwrap()) {
            progress_entry.0 += progress; // Accumulate progress
        }
    }

    // Calculate average progress for each level
    for entry in &mut level_counts {
        if let Some((total_progress, count)) = level_progress.get(&entry.level) {
            if *count > 0 {
                entry.progress = total_progress / *count as f32;
            }
        }
    }

    level_counts.retain(|synthesis| synthesis.count > 0);
    level_counts
}
