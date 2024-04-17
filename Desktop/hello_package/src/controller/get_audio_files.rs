use std::ffi::OsStr;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn all_listed_audio_files() -> Vec<PathBuf> {
    let supported_extensions = vec![
    OsStr::new("wav"),
    OsStr::new("mp3"),
    OsStr::new("flac"),
    OsStr::new("ogg"),
    OsStr::new("opus"),
    OsStr::new("aac"),
    OsStr::new("m4a"),
    OsStr::new("wma"),
    OsStr::new("alac")
];
// Find audio files in the current directory and subdirectories
let audio_files: Vec<PathBuf> = WalkDir::new(".")
    .into_iter()
    .filter_map(|entry| {
        entry.ok().and_then(|e| {
            let ext = e.path().extension()?;
            if supported_extensions.contains(&ext.to_ascii_lowercase().as_os_str()) {
                Some(e.into_path())
            } else {
                None
            }
        })
    })
    .collect();
return audio_files;

}