use super::models::{
    learning_config::LearningConfig,
    user_stat::UserStat,
    knowledge_progress::KnowledgeProgress
};
use super::calculate_progress::calculate_progress;

pub fn calculate_item_progress(config: &LearningConfig, user_stats: &Vec<UserStat>) -> Vec<KnowledgeProgress> {
    let item_progress: Vec<KnowledgeProgress> = user_stats.iter().map(|user_stat| {
        let knowledge = user_stat.knowledge.as_ref().unwrap().clone();
        let total_correct = user_stat.g;
        let total_incorrect = user_stat.w;
        let progress = calculate_progress(config, &user_stat);
        let id = knowledge.id.clone();
        KnowledgeProgress {
            id,
            knowledge,
            total_correct,
            total_incorrect,
            progress,
        }
    }).collect();

    item_progress
}
