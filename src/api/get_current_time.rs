use chrono::Utc;

pub fn get_current_time() -> i64 {
    Utc::now().timestamp()
}