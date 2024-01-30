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
    println!("Current Time: {}", current_time);

    let decay_correct = decay_factor(config, stat.updated_at, current_time, true);
    let decay_incorrect = decay_factor(config, stat.updated_at, current_time, false);
    println!("Decay Correct: {}, Decay Incorrect: {}", decay_correct, decay_incorrect);

    // Calculate retention score
    let retention_score = compute_user_stat_retention_score(config, stat);
    println!("Retention Score: {}", retention_score);

    let streak_bonus = if stat.repetitions >= Some(config.streak_bonus) { 50.0 } else { 0.0 };

    let mut score = (stat.g as f32 * decay_correct) - (stat.w as f32 * decay_incorrect) + retention_score + streak_bonus;
    println!("Initial Score: {}", score);

    if has_reached_consecutive_hours(&stat, config.consecutive_hours_threshold) {
        println!("Has reached consecutive hours threshold");
        score += score;
    }
    println!("Score after checking consecutive hours: {}", score);

    // Scale the combined score to a percentage (assuming min_value and max_value)
    let min_value = config.min_score as f32;
    let max_value = config.max_score as f32;
    let percentage = scale_to_percentage(score, min_value, max_value);
    println!("Scaled Percentage: {}", percentage);

    percentage
}
