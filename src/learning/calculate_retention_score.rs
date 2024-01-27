use crate::learning::models::{
    user_stat::UserStat,
    learning_config::LearningConfig
};
use super::get_current_time::get_current_time;
use super::calculate_decay_rate::calculate_decay_rate;

const SECONDS_IN_A_DAY: i64 = 24 * 60 * 60;

pub fn calculate_retention_score(config: &LearningConfig, user_stat: &UserStat) -> f32 {
    // Calculate elapsed time since last interaction (in days)
    let elapsed_days = (get_current_time() - user_stat.updated_at) / SECONDS_IN_A_DAY;

    // Apply decay factor based on elapsed time
    let decay_rate = calculate_decay_rate(elapsed_days, user_stat.repetitions);
    
    // Apply decay to the base retention score
    let base_score = config.progress_threshold;
    let retention_score = base_score * decay_rate as f32;

    retention_score
}
