use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct LearningConfig {
    pub decay_rate_factor_correct: f32,
    pub decay_rate_factor_incorrect: f32,
    pub consecutive_hours_threshold: i64,
    pub progress_threshold: f32,
    pub streak_bonus: i32,
    pub min_score: i32,
    pub max_score: i32,
    pub decimals: Option<usize>,
}