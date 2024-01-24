use std::path::{Path, PathBuf};

pub fn build_mp3_file_path(yaml_file_path: &Path, audio_filename: &str) -> PathBuf {
    let data_folder = yaml_file_path.parent().unwrap_or_else(|| Path::new(""));
    let audio_path = data_folder.join(audio_filename);
    audio_path
}
