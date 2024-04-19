
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn all_listed_audio_files(search_dir: &Path) -> Vec<PathBuf> {
    let supported_extensions = vec![
        OsStr::new("wav"),
        OsStr::new("mp3"),
        OsStr::new("flac"),
        OsStr::new("ogg"),
        OsStr::new("opus"),
        OsStr::new("aac"),
        OsStr::new("m4a"),
        OsStr::new("wma"),
        OsStr::new("alac"),
    ];

    let mut audio_files: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(search_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or_default();
        if supported_extensions.contains(&OsStr::new(ext)) {
            audio_files.push(path.to_path_buf());
        }
    }

    audio_files
}
