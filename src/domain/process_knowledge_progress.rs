use crate::domain::get_all_knowledge_entries::get_all_knowledge_entries;
use crate::learning::compute_knowledge_progress::compute_knowledge_progress;
use crate::learning::models::knowledge_progress::Analytic;
use crate::learning::models::{
    user_stat::UserStat,
    learning_config::LearningConfig
};
use sled::Db;

pub fn process_knowledge_progress(
    db: &Db, 
    user_stats: &[UserStat], 
    config: &LearningConfig, 
    level: Option<i32>
) -> Vec<Analytic> {
    let knowledge_entries = get_all_knowledge_entries(db);
    let knowledge_progress_map = compute_knowledge_progress(knowledge_entries, user_stats, config);

    let mut knowledges: Vec<Analytic> = knowledge_progress_map.into_iter()
        .map(|(_, kp)| kp)
        .filter(|kp| {
            if let Some(lvl) = level {
                kp.knowledge.level.map_or(false, |k_lvl| k_lvl == lvl) && kp.progress != 100.0
            } else {
                kp.progress != 100.0
            }
        })
        .collect();

    // Sort by progress (descending) then by level (ascending)
    knowledges.sort_by(|a, b| {
        match b.progress.partial_cmp(&a.progress) {
            Some(std::cmp::Ordering::Equal) => a.level.partial_cmp(&b.level).unwrap_or(std::cmp::Ordering::Equal),
            other => other.unwrap_or(std::cmp::Ordering::Equal),
        }
    });

    knowledges
}
