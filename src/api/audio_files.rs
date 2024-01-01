use rocket::get;
use rocket::fs::NamedFile;
use std::path::PathBuf;
use log::info;

#[get("/files/<file..>")]
pub async fn audio_files(file: PathBuf) -> Option<NamedFile> {
    let full_path = PathBuf::from("data").join(file);
    
     // Log the directory being accessed
     if let Some(parent) = full_path.parent() {
        info!("Accessing file in directory: {}", parent.display());
    } else {
        info!("No parent directory found for path: {}", full_path.display());
    }

    NamedFile::open(full_path).await.ok()
}