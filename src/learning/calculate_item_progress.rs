use super::models::{
    learning_config::LearningConfig,
    user_stat::UserStat,
    knowledge_progress::Analytic
};
use super::calculate_progress::calculate_progress;

pub fn calculate_item_progress(config: &LearningConfig, user_stats: &Vec<UserStat>) -> Vec<Analytic> {
    let item_progress: Vec<Analytic> = user_stats.iter().map(|user_stat| {
        let progress = calculate_progress(config, &user_stat);
        Analytic {
            id,
            progress,
        }
    }).collect();

    item_progress
}
