use rocket::get;
use rocket::fs::NamedFile;
use std::path::PathBuf;
use log::{info, error};

#[get("/files/<file..>")]
pub async fn audio_files(file: PathBuf) -> Option<NamedFile> {
    let full_path = PathBuf::from("data").join(&file);
    
    // Log the full file path being accessed
    info!("Attempting to access file at path: {}", full_path.display());

    match NamedFile::open(&full_path).await {
        Ok(file) => {
            info!("Successfully opened file: {}", full_path.display());
            Some(file)
        },
        Err(e) => {
            error!("Error opening file: {}. Error: {}", full_path.display(), e);
            None
        }
    }
}