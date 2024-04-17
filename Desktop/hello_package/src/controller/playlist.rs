use std::{collections::VecDeque, io::stdin, path::PathBuf};

use super::get_audio_files::all_listed_audio_files;
#[allow(unused_assignments)]

pub fn create_playlist(files:Vec<PathBuf>)->VecDeque<PathBuf> {
    // Read user input and create a playlist
    let audio_files: Vec<PathBuf>=all_listed_audio_files();
    let mut playlist: VecDeque<PathBuf> = VecDeque::new();
    let stdin = stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    if input.trim() == "all" {
        playlist = files.into();
    } else {
        let indices: Vec<_> = input
            .trim()
            .split(',')
            .flat_map(|s| s.trim().parse::<usize>().ok())
            .filter(|&idx| idx > 0 && idx <= audio_files.len())
            .map(|idx| audio_files[idx - 1].clone())
            .collect();

        playlist = indices.into();
    }
    println!("{:#?}",playlist);
    playlist

}