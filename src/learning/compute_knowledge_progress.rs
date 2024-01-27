use std::collections::HashMap;
use crate::learning::models::{knowledge::Knowledge, knowledge_progress::KnowledgeProgress, user_stat::UserStat, learning_config::LearningConfig};
use crate::learning::calculate_progress::calculate_progress;

pub fn compute_knowledge_progress(
    knowledge_entries: HashMap<String, Knowledge>, 
    user_stats: &[UserStat], 
    config: &LearningConfig
) -> HashMap<String, KnowledgeProgress> {
    knowledge_entries.into_iter().map(|(key_str, knowledge)| {
        let user_stat = user_stats.iter().find(|&us| us.knowledge.as_ref().unwrap().id == key_str);

        let (total_correct, total_incorrect, progress) = user_stat
            .map(|us| (us.g, us.w, calculate_progress(config, us)))
            .unwrap_or((0, 0, 0.0));
        let id = knowledge.id.clone();

        let knowledge_progress = KnowledgeProgress {
            id,
            knowledge,
            total_correct,
            total_incorrect,
            progress,
        };

        (key_str, knowledge_progress)
    }).collect()
}
