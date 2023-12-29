use rocket::{http::Header, fairing::{Fairing, Info, Kind}, config::Config, routes, get, post, State, serde::json::Json};
use rocket::{Rocket, Build, Request, Response, fs::NamedFile};
use log::LevelFilter;
use log::{info, error};
use std::{path::PathBuf, env};

use crate::models::{Database, Letter, AnswerStat, AppConfig};
use crate::domain::all_db;
use crate::utils::data_folder_path;
use crate::utils::yml_path::{load_config, get_data_folder_path};

pub struct CORS;

#[post("/content", format = "json", data = "<answer_stats>")]
fn content(dbs: &State<Database>, config: &State<AppConfig>, answer_stats: Json<Vec<AnswerStat>>) -> Json<Vec<Letter>> {
    info!("Accessing /content endpoint");

    let data_folder_path = get_data_folder_path();
    info!("Data folder path: {:?}", data_folder_path);

    let server_host = format!("http://{}:{}", config.domain, config.port);
    let static_url_path = "/files"; // The URL path that maps to your static files

    let letters_yaml_path = data_folder_path.join("letters");
    info!("Letters YAML path: {:?}", letters_yaml_path);

    let relative_path = match letters_yaml_path.strip_prefix(&data_folder_path) {
        Ok(path) => path.to_string_lossy(),
        Err(e) => {
            error!("Failed to compute relative path: {}", e);
            return Json(vec![]);  // Return an empty vector if the relative path can't be computed
        }
    };

    // let base_url = format!("{}{}/{}", server_host, static_url_path, relative_path);
    let base_url = format!("{}{}", server_host, static_url_path);

    info!("Base URL for audio files: {}", base_url);

    let db = &dbs.word_db;
    let letters = db.iter()
        .filter_map(|item| item.ok())
        .filter_map(|(key, value)| {
            match bincode::deserialize::<Letter>(&value) {
                Ok(mut letter) => {
                    let key_str = String::from_utf8_lossy(&key);
                    info!("Loaded letter with key: {}", key_str);

                    if let Some(audio_file) = &letter.audio {
                        // Construct the audio URL only if audio is not None
                        let relative_audio_path = audio_file.strip_prefix("/Users/mb/code/abjad-sled/data/letters/").unwrap_or(audio_file);
                        let audio_url = format!("{}/{}", base_url, relative_audio_path);
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

#[get("/files/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let full_path = PathBuf::from("data").join(file);
    NamedFile::open(full_path).await.ok()
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

pub async fn start_server() {
    // Set the log level based on the RUST_LOG environment variable
    env::set_var("RUST_LOG", "info"); // Adjust log level as needed: error, warn, info, debug, trace
    env_logger::Builder::from_env(env_logger::Env::default())
        .format_timestamp(None) // Disable timestamp
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

    rocket().launch().await.expect("server failed to launch");
}

#[get("/ping")]
fn ping() -> &'static str {
    "Simple content route reached"
}

fn rocket() -> Rocket<Build> {
    let data_folder_path = data_folder_path::get();
    println!("Path to wordsDB: {:?}", data_folder_path);

    let all_db = all_db::init(&data_folder_path);

    // Get the APP_ENV environment variable
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

    // Load the config based on APP_ENV
    let config_data = load_config(&app_env);

    // let mut config = Config::figment().clone();
    // config.set_port(config_data.port);

    let figment = Config::figment()
        .merge(("port", config_data.port));

    // Start the Rocket application with the custom configuration
    rocket::build()
        .configure(figment)
        .attach(CORS)
        .manage(config_data)
        .manage(all_db.clone())
        .mount("/", routes![
            content,
            files,
            ping
        ])
}