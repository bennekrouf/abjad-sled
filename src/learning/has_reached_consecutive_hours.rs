use super::{get_current_time::get_current_time, models::user_stat::UserStat};

pub fn has_reached_consecutive_hours(stat: &UserStat, consecutive_hours_threshold: i64) -> bool {
    if consecutive_hours_threshold == 0 {
        return true; // Threshold is 0, always consider it as reached
    }
    
    let current_time = get_current_time();
    let time_since_last_answer = current_time - stat.updated_at;
    time_since_last_answer <= consecutive_hours_threshold * 60 * 60 // Convert threshold to seconds
}