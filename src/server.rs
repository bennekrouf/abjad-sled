use rocket::{http::Header, fairing::{Fairing, Info, Kind}, config::Config, routes, get, State, serde::json::Json};
use rocket::{Rocket, Build, Request, Response, fs::NamedFile};
use log::LevelFilter;
use log::{info, error};

use crate::models::{Database, Letter};
use crate::domain::all_db;
use crate::utils::{data_folder_path, yml_path::load_config};
use std::{path::PathBuf, env};
pub struct CORS;

#[get("/content")]
fn content(dbs: &State<Database>) -> Json<Vec<Letter>> {
    info!("Accessing /content endpoint");

    let db = &dbs.word_db;
    let letters = db.iter()
        .filter_map(|item| item.ok())
        .filter_map(|(key, value)| {
            match bincode::deserialize::<Letter>(&value) {
                Ok(letter) => {
                    let key_str = String::from_utf8_lossy(&key);
                    info!("Loaded letter with key: {}", key_str);
                    Some(letter)
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
        .manage(all_db.clone())
        .mount("/", routes![
            content,
            files,
        ])
}