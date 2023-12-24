use crate::utils::letters_from_yaml::load;
use crate::utils::insert_in_sled;
use crate::models::Database;
use log::info;
use std::path::Path;

pub fn init(dbs: &Database) {
    let (letters, yaml_path) = load().expect("Failed to load YAML file");

    let yaml_directory = Path::new(&yaml_path).parent().unwrap_or_else(|| Path::new(""));

    for mut letter in letters {
        // If there's an audio file, prepend the relative directory path
        if let Some(audio) = letter.audio.clone() {
            let audio_path = yaml_directory.join(&audio).to_string_lossy().to_string();
            letter.audio = Some(audio_path);
        }

        let key = match &letter.name {
            Some(name) if !name.is_empty() => name,
            _ => &letter.name_arabic,  // Use `name_arabic` if `name` is `None` or empty
        };

        info!("Inserting letter with key: {}", key);
        insert_in_sled::insert_in_sled(&dbs.word_db, key, &letter);
    }
}