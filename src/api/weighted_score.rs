use crate::models::AnswerStat;
use crate::api::decay_factor::decay_factor;

pub fn weighted_score(stat: &AnswerStat, current_time: i64) -> f32 {
    let decay_correct = decay_factor(stat.updated_at, current_time, true);
    let decay_incorrect = decay_factor(stat.updated_at, current_time, false);
    (stat.g as f32 * decay_correct) - (stat.w as f32 * decay_incorrect)
}