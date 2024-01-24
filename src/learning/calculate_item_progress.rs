use super::models::{user_stat::UserStat, knowledge_progress::KnowledgeProgress};
use super::calculate_progress::calculate_progress;

pub fn calculate_item_progress(user_stats: &Vec<UserStat>, threshold: i64) -> Vec<KnowledgeProgress> {
    let item_progress: Vec<KnowledgeProgress> = user_stats.iter().map(|stat| {
        let level = stat.id.clone();
        let total_correct = stat.g;
        let total_incorrect = stat.w;
        let progress = calculate_progress(&stat, threshold);
        
        KnowledgeProgress {
            id: level,
            total_correct,
            total_incorrect,
            progress,
        }
    }).collect();

    item_progress
}
