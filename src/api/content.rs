use rocket::{post, State, serde::json::Json};
use log::{info, error};

use crate::models::{Database, Letter, AnswerStat, AppConfig};

pub struct CORS;

#[post("/content", format = "json", data = "<_answer_stats>")]
pub fn content(dbs: &State<Database>, _config: &State<AppConfig>, _answer_stats: Json<Vec<AnswerStat>>) -> Json<Vec<Letter>> {
    info!("Accessing /content endpoint");

    // let server_host = format!("http://{}", config.domain);
    // let static_url_path = "/files"; // The URL path that maps to your static files

    let db = &dbs.word_db;
    let letters = db.iter()
        .filter_map(|item| item.ok())
        .filter_map(|(key, value)| {
            match bincode::deserialize::<Letter>(&value) {
                Ok(mut letter) => {
                    let key_str = String::from_utf8_lossy(&key);
                    // info!("Loaded letter with key: {:?}", letter.audio);

                    if let Some(audio_file) = &letter.audio {
                        // Construct the correct audio URL
                        // Assuming `audio_file` contains the relative path in the database
                        // let audio_url = format!("{}{}{}", server_host, static_url_path, audio_file);
                        let audio_url = format!("{}", audio_file);
                        info!("Audio URL for letter {}: {}", key_str, audio_url);
                        letter.audio = Some(audio_url);

                        Some(letter) // Include this letter in the final vector
                    } else {
                        // Exclude this letter as its audio is None
                        None
                    }
                },
                Err(e) => {
                    error!("Failed to deserialize letter: {}", e);
                    None
                }
            }
        })
        .collect::<Vec<Letter>>();

    Json(letters)
}