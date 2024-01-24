use serde::Deserialize;
use sled::Db;
use crate::learning::models::{user_stat::UserStat, knowledge::Levelable};
use crate::learning::compute_lowest_unfinished_level::compute_lowest_unfinished_level;
use super::get_entries::get_entries;

// Function that combines get_entries and find_lowest_unfinished_level
pub fn find_lowest_unfinished_level<T>(db: &Db, user_stats: &[UserStat], threshold: i64) -> Option<i32>
    where T: PartialEq + Clone + for<'de> Deserialize<'de> + Levelable,
{
    let entries = get_entries::<T>(db, user_stats);
    compute_lowest_unfinished_level::<T>(&entries, user_stats, threshold)
}
