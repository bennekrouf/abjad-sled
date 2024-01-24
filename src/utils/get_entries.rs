use std::collections::HashMap;
use sled::{Db, Iter};
use crate::learning::models::{user_stat::UserStat, knowledge::Levelable};
use crate::utils::read_from_sled::read_from_sled;
use serde::de::DeserializeOwned;

// Function to retrieve and deserialize key-value pairs from the database
pub fn get_entries<T>(db: &Db, user_stats: &[UserStat]) -> HashMap<String, T>
    where T: PartialEq + Clone + Levelable, T: DeserializeOwned,
{
    let mut entries: HashMap<String, T> = HashMap::new();

    // Use sled::Iter to iterate over entries in the database
    let iter: Iter = db.iter();

    for item in iter {
        if let Ok((key, _)) = item {
            let key_str = String::from_utf8_lossy(&key).to_string();

            // Use the get_and_deserialize_from_sled function to retrieve and deserialize the value
            if let Some(item) = read_from_sled::<T, _>(&db, &key) {
                if let Some(_) = user_stats.iter().find(|s| s.id == key_str) {
                    entries.insert(key_str, item);
                }
            }
        }
    }

    entries
}