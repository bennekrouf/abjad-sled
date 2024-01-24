mod models;
mod validator;
mod server;
mod xhr_guard;
mod learning;

mod domain;
mod api;
mod utils;

#[tokio::main]
async fn main() {
    // Check deserialization of all files before starting the server
    if let Err(e) = validator::validate() {
        eprintln!("Similar error loading validator files: {}", e);
        std::process::exit(1);
    }
    server::start_server().await;
}