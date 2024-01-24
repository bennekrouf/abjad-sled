use bincode;
use serde::Serialize;
use sled::Tree;

pub fn insert_in_sled<T, K>(db: &Tree, key: K, value: &T)
    where T: Serialize, K: Clone + AsRef<[u8]>,
{
    let serialized_value = bincode::serialize(value).unwrap();
    
    db
        .insert(key, serialized_value)
        .expect("Failed to insert into db");
}