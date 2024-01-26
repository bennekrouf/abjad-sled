use serde::Deserialize;
use sled::Db;
use crate::learning::models::{user_stat::UserStat, knowledge::Levelable};
use crate::learning::compute_lowest_unfinished_level::compute_lowest_unfinished_level;
use super::get_entries::get_entries;
use crate::learning::models::learning_config::LearningConfig;

// Function that combines get_entries and find_lowest_unfinished_level
pub fn find_lowest_unfinished_level<T>(config: &LearningConfig, db: &Db, user_stats: &[UserStat]) -> Option<i32>
    where T: PartialEq + Clone + for<'de> Deserialize<'de> + Levelable,
{
    let entries = get_entries::<T>(db, user_stats);
    compute_lowest_unfinished_level::<T>(config, &entries, user_stats)
}
