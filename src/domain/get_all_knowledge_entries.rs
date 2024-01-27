use std::collections::HashMap;
use sled::Db;
use crate::learning::models::knowledge::Knowledge;
use crate::utils::read_from_sled::read_from_sled;

pub fn get_all_knowledge_entries(db: &Db) -> HashMap<String, Knowledge> {
    db.iter()
        .filter_map(Result::ok)
        .filter_map(|(key, _)| {
            read_from_sled::<Knowledge, _>(&db, &key)
        })
        .filter(|knowledge| knowledge.audio.is_some())  // Filter out entries without audio
        .map(|knowledge| (knowledge.id.clone(), knowledge))
        .collect()
}