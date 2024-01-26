use crate::learning::models::user_stat::UserStat;
use crate::learning::decay_factor::decay_factor;
use crate::learning::get_current_time::get_current_time;
use super::has_reached_consecutive_hours::has_reached_consecutive_hours;
use super::calculate_retention_score::calculate_retention_score;
use super::scale_to_percentage::scale_to_percentage;
use super::models::learning_config::LearningConfig;

pub fn calculate_progress(config: &LearningConfig, stat: &UserStat) -> f32 {
    let current_time = get_current_time();
    let decay_correct = decay_factor(config, stat.updated_at, current_time, true);
    let decay_incorrect = decay_factor(config, stat.updated_at, current_time, false);
    
    // Calculate retention score
    let retention_score = calculate_retention_score(stat);

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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::learning::decay_factor::decay_factor;

    // Mock function for get_current_time
    fn mock_get_current_time() -> i64 {
        // Return a fixed time value for testing
        1000000 
    }

    // Test the calculate_progress function
    #[test]
    fn test_calculate_progress() {
        // Create a UserStat instance
        let user_stat = UserStat {
            id: "test_id".to_string(),
            g: 10, // 10 correct answers
            w: 5,  // 5 wrong answers
            updated_at: 999000, // Some past time
        };

        // Use the mock_get_current_time instead of the real one
        let current_time = mock_get_current_time();
        let decay_correct = decay_factor(user_stat.updated_at, current_time, true);
        let decay_incorrect = decay_factor(user_stat.updated_at, current_time, false);

        // Calculate expected progress
        let expected_progress = (user_stat.g as f32 * decay_correct) - (user_stat.w as f32 * decay_incorrect);

        // Assert that calculate_progress returns the expected result
        assert_eq!(calculate_progress(&user_stat), expected_progress);
    }

    // Mock function for get_current_time
    fn mock_get_current_time_short_interval() -> i64 {
        // Simulate current time as a short interval after updated_at
        100000 + (30 * 24 * 60 * 60) // 30 days later
    }

    // Test that 3 correct answers give a progress of more than 90%
    #[test]
    fn test_three_correct_answers_high_progress() {
        // Create a UserStat instance with 3 correct answers and 0 wrong answers
        let user_stat = UserStat {
            id: "test_id".to_string(),
            g: 3, // 3 correct answers
            w: 0,  // 0 wrong answers
            updated_at: 100000, // Some past time
        };

        // Use the mock_get_current_time_short_interval instead of the real one
        let current_time = mock_get_current_time_short_interval();
        let decay_correct = decay_factor(user_stat.updated_at, current_time, true);

        // Calculate actual progress
        let actual_progress = (user_stat.g as f32 * decay_correct) - (user_stat.w as f32 * decay_factor(user_stat.updated_at, current_time, false));
    
        // Temporarily print out the decay factor for debugging
        println!("Decay factor for correct answers: {}", decay_correct);

        // Assert that progress is more than 90%
        assert!(actual_progress > 90.0, "Progress is not more than 90%, actual: {}", actual_progress);
    }
}
