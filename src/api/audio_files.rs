use rocket::get;
use rocket::fs::NamedFile;
use std::path::PathBuf;
use log::{info, error};
use crate::utils::yml_path::get_data_folder_path;

#[get("/files/<file..>")]
pub async fn audio_files(file: PathBuf) -> Option<NamedFile> {
    let base_path = get_data_folder_path(); // Get the base data directory
    let full_path = base_path.join(file); // Join with the requested file path

    // Log the attempted access path
    info!("Attempting to access file at path: {}", full_path.display());

    match NamedFile::open(full_path).await {
        Ok(file) => Some(file),
        Err(e) => {
            error!("Error opening file: {}", e);
            None
        }
    }
}