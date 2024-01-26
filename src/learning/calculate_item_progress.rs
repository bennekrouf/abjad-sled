use super::models::{learning_config::LearningConfig, user_stat::UserStat, knowledge_progress::KnowledgeProgress};
use super::calculate_progress::calculate_progress;

pub fn calculate_item_progress(config: &LearningConfig, user_stats: &Vec<UserStat>) -> Vec<KnowledgeProgress> {
    let item_progress: Vec<KnowledgeProgress> = user_stats.iter().map(|stat| {
        let level = stat.id.clone();
        let total_correct = stat.g;
        let total_incorrect = stat.w;
        let progress = calculate_progress(config, &stat);
        
        KnowledgeProgress {
            id: level,
            total_correct,
            total_incorrect,
            progress,
        }
    }).collect();

    item_progress
}
