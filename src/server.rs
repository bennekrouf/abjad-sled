use rocket::{http::Header, fairing::{Fairing, Info, Kind}, config::Config, routes, get, post, State, serde::json::Json};
use rocket::{Rocket, Build, Request, Response, fs::NamedFile};
use log::LevelFilter;
use log::{info, error};
use std::{path::PathBuf, env};

use crate::models::{Database, Letter, AnswerStat, AppConfig};
use crate::domain::all_db;
use crate::utils::data_folder_path;
use crate::utils::yml_path::load_config;

pub struct CORS;

#[post("/content", format = "json", data = "<_answer_stats>")]
fn content(dbs: &State<Database>, _config: &State<AppConfig>, _answer_stats: Json<Vec<AnswerStat>>) -> Json<Vec<Letter>> {
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
    // Get the APP_ENV environment variable
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

    // Load the config based on APP_ENV
    let config_data = load_config(&app_env);
    let all_db = all_db::init(&data_folder_path, &config_data);

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