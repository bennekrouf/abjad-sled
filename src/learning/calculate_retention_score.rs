use crate::learning::models::user_stat::UserStat;
use super::get_current_time::get_current_time;
use super::calculate_decay_rate::calculate_decay_rate;

const SECONDS_IN_A_DAY: i64 = 24 * 60 * 60;
const BASE_RETENTION_SCORE: f32 = 100.0; // higher means easier to recall

pub fn calculate_retention_score(user_stat: &UserStat) -> f32 {
    // Calculate elapsed time since last interaction (in days)
    let elapsed_days = (get_current_time() - user_stat.last_answered_at) / SECONDS_IN_A_DAY;

    // Apply decay factor based on elapsed time
    let decay_rate = calculate_decay_rate(elapsed_days, user_stat.repetitions);
    
    // Apply decay to the base retention score
    let base_score = BASE_RETENTION_SCORE;
    let retention_score = base_score * decay_rate;

    retention_score
}
