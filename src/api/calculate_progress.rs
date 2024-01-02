use crate::models::AnswerStat;

use super::weighted_score::weighted_score;

pub fn calculate_progress(stat: &AnswerStat, current_time: i64) -> f32 {
    weighted_score(stat, current_time)
}