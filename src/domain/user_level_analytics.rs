use crate::learning::models::user_stat::UserStat;
use crate::models::Database;
use crate::learning::models::{
    knowledge::Knowledge,
    level_analytics::LevelAnalytics,
    learning_config::LearningConfig
};
use crate::learning::compute_user_stat_progress::compute_user_stat_progress;

pub fn user_level_analytics(config: &LearningConfig, db: &Database, user_stats: &Vec<UserStat>) -> Vec<LevelAnalytics> {
    let mut level_counts: Vec<LevelAnalytics> = db.word_db.iter()
        .filter_map(|item| item.ok())
        .filter_map(|(_, data)| bincode::deserialize::<Knowledge>(&data).ok())
        .fold(std::collections::HashMap::new(), |mut counts, item| {
            let entry = counts.entry(item.level).or_insert_with(|| LevelAnalytics {
                level: item.level.unwrap(),
                count: 0,
                progress: 0.0,
            });
            entry.count += 1;
            counts
        })
        .into_iter()
        .map(|(_, synthesis)| synthesis)
        .collect();

    for (stat, item) in user_stats.iter()
        .flat_map(|stat| {
            db.word_db.iter()
                // .find(|item| item.as_ref().ok().map_or(false, |(key, _)| key == stat.id))
                .find(|item| item.as_ref().ok().map_or(false, |(key, _)| String::from_utf8_lossy(key).as_ref() == stat.id))
                .and_then(|item| bincode::deserialize::<Knowledge>(&item.unwrap().1).ok())
                .map(|item| (stat, item))
        }) {
        if let Some(entry) = level_counts.iter_mut().find(|synthesis| synthesis.level == item.level.unwrap()) {
            entry.progress += compute_user_stat_progress(config, &stat);
        }
    }

    level_counts.retain(|synthesis| synthesis.count > 0);
    level_counts
}