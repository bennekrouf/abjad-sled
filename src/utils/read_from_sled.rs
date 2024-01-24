use serde::de::DeserializeOwned;
use sled::Tree;

pub fn read_from_sled<T, K>(db: &Tree, key: K) -> Option<T>
    where T: DeserializeOwned, K: AsRef<[u8]>,
{
    if let Some(serialized_value) = db.get(key).expect("Failed to get from db") {
        if let Ok(deserialized_value) = bincode::deserialize(&serialized_value) {
            return Some(deserialized_value);
        }
    }
    None
}
