use crate::domain::knowledge_entries::knowledge_entries;
use crate::learning::models::analytic::Analytic;
use crate::learning::models::{
    user_stat::UserStat,
    learning_config::LearningConfig
};
use crate::learning::compute_user_stat_progress::compute_user_stat_progress;
use sled::Db;

pub fn user_analytics(
    db: &Db, 
    user_stats: &[UserStat], 
    config: &LearningConfig, 
    level: Option<i32>
) -> Vec<Analytic> {
    let knowledge_entries = knowledge_entries(db);
    let knowledge_progress_map:(String, Analytic) = knowledge_entries.into_iter().map(|(key_str, knowledge)| {
            let user_stat = user_stats.iter().find(|&us| us.id == key_str);

            let (progress) = user_stat
                .map(|us| (compute_user_stat_progress(config, us)))
                .unwrap_or((0.0));
            let id = knowledge.id.clone();

            let analytic = Analytic {
                id,
                progress,
            };

            (key_str, analytic)
        }).collect();

    let mut analytics: Vec<Analytic> = knowledge_progress_map.into_iter()
        .map(|(_, analytic)| analytic)
        .filter(|analytic| {
            if let Some(lvl) = level {
                analytic.knowledge.level.map_or(false, |k_lvl| k_lvl == lvl) && analytic.progress != 100.0
            } else {
                analytic.progress != 100.0
            }
        })
        .collect();

    // Sort by progress (descending) then by level (ascending)
    analytics.sort_by(|a, b| {
        match b.progress.partial_cmp(&a.progress) {
            Some(std::cmp::Ordering::Equal) => a.level.partial_cmp(&b.level).unwrap_or(std::cmp::Ordering::Equal),
            other => other.unwrap_or(std::cmp::Ordering::Equal),
        }
    });

    analytics
}
