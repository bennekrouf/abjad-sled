use rocket::{http::Header, fairing::{Fairing, Info, Kind}, config::Config, routes};
use rocket::{Rocket, Build, Request, Response};
use log::LevelFilter;
use std::env;

use crate::{domain::all_db, api::level_count::level_count};
use crate::utils::data_folder_path;
// use crate::utils::yml_path;
use crate::utils::yml_path::{LEARNING, CONFIG};

use crate::api::content::content;
use crate::api::audio_files::audio_files;
use crate::api::ping::ping;
use crate::api::level_count_detail::level_count_detail;


pub struct CORS;


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
    env::set_var("RUST_LOG", "info"); // error, warn, info, debug, trace
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

    // Load both configurations
    let app_config = CONFIG.lock().unwrap().clone();
    let learning_config = LEARNING.lock().unwrap().clone();

    let all_db = all_db::init(&data_folder_path, &app_config);
    let figment = Config::figment()
        .merge(("port", app_config.port));

    rocket::build()
        .configure(figment)
        .attach(CORS)
        .manage(all_db.clone())
        .manage(app_config) // Manage app_config
        .manage(learning_config) // Manage learning_config
        .mount("/", routes![
            content,
            audio_files,
            ping,
            level_count,
            level_count_detail
        ])
}