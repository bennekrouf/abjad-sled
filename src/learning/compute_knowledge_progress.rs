use std::collections::HashMap;
use crate::learning::models::{knowledge::Knowledge, knowledge_progress::Analytic, user_stat::UserStat, learning_config::LearningConfig};
use crate::learning::calculate_progress::calculate_progress;

pub fn compute_knowledge_progress(
    knowledge_entries: HashMap<String, Knowledge>, 
    user_stats: &[UserStat], 
    config: &LearningConfig
) -> HashMap<String, Analytic> {
    knowledge_entries.into_iter().map(|(key_str, knowledge)| {
        let user_stat = user_stats.iter().find(|&us| us.knowledge.as_ref().unwrap().id == key_str);

        let (progress) = user_stat
            .map(|us| (calculate_progress(config, us)))
            .unwrap_or((0.0));
        let id = knowledge.id.clone();

        let knowledge_progress = Analytic {
            id,
            progress,
        };

        (key_str, knowledge_progress)
    }).collect()
}
