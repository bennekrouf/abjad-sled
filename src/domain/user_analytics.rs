use std::collections::HashMap;
use crate::domain::knowledge_entries::knowledge_entries;
use crate::learning::models::{
    user_stat::UserStat,
    learning_config::LearningConfig,
    analytic::Analytic
};
use crate::learning::compute_user_stat_progress::compute_user_stat_progress;
use sled::Db;

pub fn user_analytics(
    db: &Db,
    user_stats: &[UserStat],
    config: &LearningConfig, 
    level: Option<u8>
) -> Vec<Analytic> {
    let knowledge_entries = knowledge_entries(db, level);

    // Initialize analytics for all knowledge entries with default progress
    let mut analytics_map: HashMap<String, Analytic> = knowledge_entries.iter().map(|(id, _)| {
        (id.clone(), Analytic { id: id.clone(), progress: 0.0, category: None })
    }).collect();

    // Update progress for analytics based on user stats
    for us in user_stats {
        if let Some(_) = knowledge_entries.get(&us.id) {
            if let Some(analytic) = analytics_map.get_mut(&us.id) {
                analytic.progress = compute_user_stat_progress(config, us);
            }
        }
    }

    // Convert HashMap to Vec and apply any additional filtering or sorting
    let analytics: Vec<Analytic> = analytics_map.into_values().collect();

    // Filter out analytics with progress not equal to 100.0, if level is specified
    let mut filtered_analytics: Vec<Analytic> = if level.is_some() {
        analytics.into_iter().filter(|analytic| {
            knowledge_entries.get(&analytic.id).map_or(false, |knowledge| {
                knowledge.level.map_or(false, |k_lvl| k_lvl == level.unwrap()) && analytic.progress != 100.0
            })
        }).collect()
    } else {
        analytics
    };

    // Sort by progress (descending) then by level (ascending)
    filtered_analytics.sort_by(|a, b| {
        let a_level = knowledge_entries.get(&a.id).and_then(|k| k.level).unwrap_or(0);
        let b_level = knowledge_entries.get(&b.id).and_then(|k| k.level).unwrap_or(0);

        match b.progress.partial_cmp(&a.progress) {
            Some(std::cmp::Ordering::Equal) => a_level.cmp(&b_level),
            other => other.unwrap_or(std::cmp::Ordering::Equal),
        }
    });

    filtered_analytics
}
