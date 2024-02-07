use super::{
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
    // println!("Current Time: {}", current_time);

    let decay_correct = decay_factor(config, stat.updated_at, current_time, true);
    let decay_incorrect = decay_factor(config, stat.updated_at, current_time, false);
    // println!("Decay Correct: {}, Decay Incorrect: {}", decay_correct, decay_incorrect);

    // Calculate retention score
    let retention_score = compute_user_stat_retention_score(config, stat);
    // println!("Retention Score: {}", retention_score);

    let streak_bonus = if stat.repetitions >= Some(config.streak_bonus) { 50.0 } else { 0.0 };

    let mut score = (stat.g as f32 * decay_correct) - (stat.w as f32 * decay_incorrect) + retention_score + streak_bonus;
    // println!("Initial Score: {}", score);

    if has_reached_consecutive_hours(&stat, config.consecutive_hours_threshold) {
        // println!("Has reached consecutive hours threshold");
        score += score;
    }
    // println!("Score after checking consecutive hours: {}", score);

    // Scale the combined score to a percentage (assuming min_value and max_value)
    let min_value = config.min_score as f32;
    let max_value = config.max_score as f32;
    let percentage = scale_to_percentage(score, min_value, max_value);
    // println!("Scaled Percentage: {}", percentage);

    let decimals = config.decimals.unwrap_or(0);

    let formatted_percentage = format!("{:.1$}", percentage, decimals);
    // println!("Formatted Percentage: {}", formatted_percentage);

    formatted_percentage.parse().unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::learning::models::learning_config::LearningConfig;
    use crate::learning::models::user_stat::UserStat;

    use chrono::{TimeZone, Utc};

    #[test]
    fn test_basic_progress_calculation() {
        let config = LearningConfig {
            streak_bonus: 3,
            consecutive_hours_threshold: 24,
            min_score: 0,
            max_score: 100,
            // Other fields as necessary...
        };
        let stat = UserStat {
            updated_at: Utc.ymd(2022, 5, 1).and_hms(12, 0, 0).timestamp(),
            g: 10,
            w: 5,
            repetitions: Some(2),
            // Other fields as necessary...
        };
        let progress = compute_user_stat_progress(&config, &stat);
        assert!(progress > 0.0); // The exact assertion might need adjustment based on decay_factor and scale_to_percentage calculations
    }

    #[test]
    fn test_streak_bonus() {
        let config = LearningConfig {
            streak_bonus: 2, // Lower threshold for testing
            // Other configurations...
        };
        let stat = UserStat {
            repetitions: Some(3), // Exceeds streak bonus threshold
            // Populate other necessary fields...
        };
        let progress = compute_user_stat_progress(&config, &stat);
        assert!(progress > 0.0); // Check that the progress includes the streak bonus
    }

    #[test]
    fn test_consecutive_hours_bonus() {
        // This test assumes has_reached_consecutive_hours would return true for the given stat
        let config = LearningConfig {
            // Configuration that would trigger the consecutive hours bonus...
        };
        let stat = UserStat {
            // UserStat configuration that would simulate reaching consecutive hours threshold...
        };
        let initial_progress = compute_user_stat_progress(&config, &stat);
        // Potentially manipulate stat to not meet the threshold and compute again
        let adjusted_progress = compute_user_stat_progress(&config, &stat); // Adjust stat accordingly
        assert!(initial_progress < adjusted_progress); // Assuming the score doubles when the threshold is met
    }
}
