use std::fs::File;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::io::{self, BufRead};
use rodio::{Decoder, OutputStream, Sink, Source};

use crate::controller::get_audio_files:: all_listed_audio_files;
use crate::controller::playlist::create_playlist;
use crate::directories::directory;
pub fn play () {
       //scan and get all audio files from your files system
       let audio_files = all_listed_audio_files(&directory());
       //list of playlist chosen by the user
    let list = create_playlist(audio_files);

    let playlist = Arc::new(Mutex::new(list));
    let current_song = Arc::new(Mutex::new(0));
    let condvar = Arc::new(Condvar::new());

    let playlist_clone = Arc::clone(&playlist);
    let current_song_clone = Arc::clone(&current_song);
    let condvar_clone = Arc::clone(&condvar);
//using radio now 
let(_stream, stream_handle)=OutputStream::try_default().unwrap();
let sink=Sink::try_new(&stream_handle).unwrap();

    let audio_thread = thread::spawn(move || {
        loop {
            let playlist = playlist_clone.lock().unwrap();
            let mut current_song = *current_song_clone.lock().unwrap();
            if current_song < playlist.len() {
                println!("Playing: {:?}", playlist[current_song]);
                //playing the song
                let source = Decoder::new(File::open(&playlist[current_song]).unwrap()).unwrap();
                sink.append(source);
                
               //check for any notification
              
                if let Ok(mut current_song) = condvar_clone.wait(current_song_clone.lock().unwrap()) {
                    sink.stop();
                    // If there's a change, update current_song and continue
                    current_song = current_song;
                }

                if sink.empty() {
                    println!("Song ended, moving to the next song...");
                    current_song += 1;
                    // Ensure the loop continues to the next song
                    continue;

                }
               
            } else {
                println!("Playlist finished");
                break;
            }
        }
    });

    let user_input_thread = thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            match line.as_str() {
                "n" => {
                    let mut current_song = current_song.lock().unwrap();
                    *current_song += 1;
                    condvar.notify_one(); // Notify the audio thread of the change
                }
                "p" => {
                    let mut current_song = current_song.lock().unwrap();
                    if *current_song > 0 {
                        *current_song -= 1;
                    }
                    condvar.notify_one(); // Notify the audio thread of the change
                }
                _ => println!("Unknown command"),
            }
        }
    });

    audio_thread.join().unwrap();
    user_input_thread.join().unwrap();
}
