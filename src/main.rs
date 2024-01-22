pub mod models;
mod validator;
mod server;
mod xhr_guard;
mod learning;

pub mod domain {
    pub mod all_db;
    pub mod word_init;
}
pub mod api {
    pub mod content;
    pub mod audio_files;
    pub mod ping;
    pub mod level_count;
}

pub mod utils {
    pub mod yml_path;
    pub mod data_folder_path;
    pub mod insert_in_sled;
    pub mod build_mp3_file_url;
    pub mod build_mp3_file_path;
    pub mod data_from_yaml;
}

#[tokio::main]
async fn main() {
    // Check deserialization of all files before starting the server
    if let Err(e) = validator::validate() {
        eprintln!("Similar error loading validator files: {}", e);
        std::process::exit(1);
    }
    server::start_server().await;
}