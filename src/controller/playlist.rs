use std::{collections::VecDeque, io::stdin, path::PathBuf};

use crate::{helpers::get_directories::directory, helpers::file_utils::get_file_name};

use super::get_audio_files::all_listed_audio_files;
#[allow(unused_assignments)]

pub fn create_playlist(files:Vec<PathBuf>)->VecDeque<PathBuf> {
    // Read user input and create a playlist
    let audio_files: Vec<PathBuf>=all_listed_audio_files(&directory());
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
    println!("Playlist created with {} songs.", playlist.len());
    playlist.iter().enumerate().for_each(|(index, path)| {
        
        if let Some(file_name) = get_file_name(path) {
            println!("{}. {}", index + 1, file_name);
        } else {
            println!("{}. No name", index + 1);
        }
    });
    playlist

}