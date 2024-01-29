use super::models::{
    learning_config::LearningConfig,
    user_stat::UserStat,
    analytic::Analytic
};
use super::compute_user_stat_progress::compute_user_stat_progress;

pub fn compute_user_stats_analytics(config: &LearningConfig, user_stats: &Vec<UserStat>) -> Vec<Analytic> {
    let analytics: Vec<Analytic> = user_stats.iter().map(|user_stat| {
        let progress = compute_user_stat_progress(config, &user_stat);
        Analytic {
            id: user_stat.id.clone(),
            progress,
        }
    }).collect();

    analytics
}
