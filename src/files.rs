use std::fs;
use std::path::PathBuf;

pub fn get_audio_files(dir: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext == "wav" || ext == "mp3" {
                    files.push(path);
                }
            }
        }
    }
    files
}
