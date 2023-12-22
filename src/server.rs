use rocket::{routes, get, serde::{json::Json}};
use rocket::config::Config;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Rocket, Build, Request, Response};
use std::{fs, env};
use serde::Serialize;
use log::LevelFilter;

use crate::domain::all_db;
use crate::utils::data_folder_path;
use crate::utils::yml_path::load_config;
// use path::Path;
// use time::now;
use std::path::PathBuf;
// use rocket::http::uri::Path;
use rocket::fs::NamedFile;
// use rocket::yansi::Paint;
pub struct CORS;

#[derive(Serialize)]
struct Mp3File {
    name: String,
    url: String,
}

#[get("/content")]
fn content() -> Json<Vec<Mp3File>> {
    let mp3_files = fs::read_dir("data")
        .expect("Failed to read data directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "mp3"))
        .map(|entry| {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            Mp3File {
                name: file_name.clone(),
                url: format!("http://localhost:7000/files/{}", file_name),
            }
        })
        .collect::<Vec<Mp3File>>();

    Json(mp3_files)
}

#[get("/files/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let full_path = PathBuf::from("data").join(file);
    NamedFile::open(full_path).await.ok()

    // NamedFile::open(Path::new("data/").join(file)).await.ok()
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