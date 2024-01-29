use crate::learning::{
    models::user_stat::UserStat,
    decay_factor::decay_factor,
    get_current_time::get_current_time
};
use super::has_reached_consecutive_hours::has_reached_consecutive_hours;
use super::compute_user_stat_retention_score::compute_user_stat_retention_score;
use super::scale_to_percentage::scale_to_percentage;
use super::models::learning_config::LearningConfig;

pub fn compute_user_stat_progress(config: &LearningConfig, stat: &UserStat) -> f32 {
    let current_time = get_current_time();
    let decay_correct = decay_factor(config, stat.updated_at, current_time, true);
    let decay_incorrect = decay_factor(config, stat.updated_at, current_time, false);
    
    // Calculate retention score
    let retention_score = compute_user_stat_retention_score(config, stat);

    let mut score = (stat.g as f32 * decay_correct) - (stat.w as f32 * decay_incorrect) + retention_score;

    if has_reached_consecutive_hours(&stat, config.consecutive_hours_threshold) {
        score += score;
    }

    // Scale the combined score to a percentage (assuming min_value and max_value)
    let min_value = 2.0;
    let max_value = 5.0;
    let percentage = scale_to_percentage(score, min_value, max_value);

    percentage
}