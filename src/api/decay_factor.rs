use crate::utils::yml_path;

pub fn decay_factor(updated_at: i64, current_time: i64, is_correct: bool) -> f32 {
    let config = yml_path::CONFIG.lock().unwrap();
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