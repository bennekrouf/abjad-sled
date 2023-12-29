pub mod models;
mod validator;
mod server;
mod xhr_guard;

pub mod domain {
    pub mod all_db;
    pub mod word_init;
}
pub mod utils {
    pub mod yml_path;
    pub mod data_folder_path;
    pub mod letters_from_yaml;
    pub mod insert_in_sled;
    pub mod build_mp3_file_url;
    pub mod build_mp3_file_path;
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