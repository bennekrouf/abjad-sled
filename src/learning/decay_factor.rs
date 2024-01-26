// calculates the reduction (decay) of learning retention 
// or memory strength over a period of time
// How much value is lost per unit of time, adjusted per year the applied to the 
// actual time difference
// Between 0.0 and 1.0, represents a proportion of the original value that remains
// after decay over time

// example :
// - current time is 6 months after updated_at, computed on incorrect answer, 
// decay of 0.67 means 
// that 67% of the original learning retention remains after 6 months assuming the last
// answer is incorrect
// 1 means 100% retention (no decay), 0 means 0% retention (complete decay)

use super::models::learning_config::LearningConfig;

pub fn decay_factor(config: &LearningConfig, updated_at: i64, current_time: i64, is_correct: bool) -> f32 {
    let decay_rate_factor = if is_correct {
        config.decay_rate_factor_correct
    } else {
        config.decay_rate_factor_incorrect
    };

    let time_diff = current_time - updated_at;
    let one_year_in_seconds = 365 * 24 * 60 * 60;
    let decay_rate = decay_rate_factor / one_year_in_seconds as f32;

    let decay = (-decay_rate * time_diff as f32).exp();
    decay.clamp(0.0, 1.0)
}