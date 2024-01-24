use chrono::Utc;
use super::models::user_stat::UserStat;

pub fn has_reached_consecutive_hours(stat: &UserStat, threshold: i64) -> bool {
    if threshold == 0 {
        return true; // Threshold is 0, always consider it as reached
    }
    
    let current_time = Utc::now().timestamp(); // Get current timestamp
    let time_since_last_answer = current_time - stat.updated_at;
    time_since_last_answer <= threshold * 60 * 60 // Convert threshold to seconds
}