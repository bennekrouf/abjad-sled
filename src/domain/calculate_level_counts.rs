use crate::learning::models::user_stat::UserStat;
use crate::models::Database;
use crate::learning::models::{
    knowledge::Knowledge,
    level_count::LevelCount,
    learning_config::LearningConfig
};
use crate::learning::calculate_progress::calculate_progress;

pub fn calculate_level_counts(config: &LearningConfig, db: &Database, user_stats: &Vec<UserStat>) -> Vec<LevelCount> {
    let mut level_counts: Vec<LevelCount> = db.word_db.iter()
        .filter_map(|item| item.ok())
        .filter_map(|(_, data)| bincode::deserialize::<Knowledge>(&data).ok())
        .fold(std::collections::HashMap::new(), |mut counts, item| {
            let entry = counts.entry(item.level).or_insert_with(|| LevelCount {
                level: item.level.unwrap(),
                count: 0,
                total_correct: 0,
                total_incorrect: 0,
                progress: 0.0,
                total_score: 0.0,
            });
            entry.count += 1;
            counts
        })
        .into_iter()
        .map(|(_, level_count)| level_count)
        .collect();

    for (stat, item) in user_stats.iter()
        .flat_map(|stat| {
            db.word_db.iter()
                .find(|item| item.as_ref().ok().map_or(false, |(key, _)| key == stat.knowledge.as_ref().unwrap().id.as_bytes()))
                .and_then(|item| bincode::deserialize::<Knowledge>(&item.unwrap().1).ok())
                .map(|item| (stat, item))
        }) {
        if let Some(entry) = level_counts.iter_mut().find(|level_count| level_count.level == item.level.unwrap()) {
            entry.total_correct += stat.g;
            entry.total_incorrect += stat.w;
            entry.total_score += calculate_progress(config, &stat);
        }
    }

    level_counts.retain(|level_count| level_count.count > 0);
    level_counts
}