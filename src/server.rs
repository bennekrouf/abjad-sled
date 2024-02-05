use rocket::{http::Header, fairing::{Fairing, Info, Kind}, config::Config, routes};
use rocket::{Rocket, Build, Request, Response};
use log::LevelFilter;
use std::env;

use crate::{
    api::user_content::user_content,
    api::level_analytics::level_analytics,
    domain::all_db, utils::{yml_path::{LEARNING, CONFIG},data_folder_path}
};

use crate::api::{
    audio_files::audio_files,
    ping::ping,
    get_knowledge_entries::get_knowledge_entries,
    user_stats_analytics::user_stats_analytics,
};

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
    
    let figment = Config::figment()
    .merge(("port", app_config.port));
    
    let all_db = all_db::init(&data_folder_path, &app_config);
    
    rocket::build()
        .configure(figment)
        .attach(CORS)
        .manage(all_db.clone())
        .manage(app_config)
        .manage(learning_config)
        .mount("/", routes![
            user_content,
            audio_files,
            ping,
            user_stats_analytics,
            level_analytics,
            get_knowledge_entries
        ])
}