use std::fs::File;
use std::sync::{ Arc, Mutex, Condvar };
use std::thread::{self, current};
use std::io::{  stdin, BufRead };
use rodio::{ Decoder, OutputStream, Sink};

use super::controller::get_audio_files::all_listed_audio_files;
use super::controller::playlist::create_playlist;
use super::directories::directory;
pub fn play() {
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
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Arc::new(Sink::try_new(&stream_handle).unwrap());
    let sink_clone = Arc::clone(&sink);
    let audio_thread = thread::spawn(move || {
        loop {
            let playlist = playlist_clone.lock().unwrap();
            let mut current_song = *current_song_clone.lock().unwrap();
            if current_song < playlist.len() {
                println!("Playing: {:?}", playlist[current_song]);
                //playing the song
                let source = Decoder::new(File::open(&playlist[current_song]).unwrap()).unwrap();

                sink.append(source);
                // sink.sleep_until_end();

                println!("current song is {:?}", current_song);

                while sink.empty()==false {
                    let mut current_song_guard = current_song_clone.lock().unwrap();
                    if *current_song_guard !=current_song {
                        println!("executed on play");
                        current_song=*current_song_guard;
                        println!(" guard :{}",current_song_guard);
                        println!(" curent song :{}",current_song);
                   
                        break;
                     
                    }
                   

                }
               while sink.empty() == true {
             
                     println!("Song ended, moving to the next song...");
                   
                    println!("why not executed");
                    current_song += 1;
                    condvar_clone.notify_one();
                    let mut current_song_guard = current_song_clone.lock().unwrap();
                     if *current_song_guard !=current_song {
                        println!("executed on play");
                       current_song=*current_song_guard;
                        println!(" guard :{}",current_song_guard);
                        println!(" curent song :{}",current_song);
                   
                        break;
                     
                    }
                   
                    println!("{}",current_song);


                }
               

                // current_song += 1;

            } else {
                println!("Playlist finished");
                break;
            }
        }
    });

    let user_input_thread = thread::spawn(move || {
        
        let stdin = stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            match line.as_str() {
                "n" => {
                    let mut current_song = current_song.lock().unwrap();
                    sink_clone.stop();
                    // sink_clone.stop();
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
                "k" => {
                    if sink_clone.is_paused() {
                        println!("Resumed");
                        sink_clone.play();
                    } else {
                        println!("Paused");
                        sink_clone.pause();
                    }
                }
                "q" => {
                    println!("are you sure you want to quit");
                    let stdn = std::io::stdin();
                    let mut input = String::new();
                    stdn.lock().read_line(&mut input).unwrap();
                    if input.trim() == "y" {
                        println!("Quitting...");

                        condvar.notify_one();
                        break;
                    }
                }
                _ => println!("Unknown command"),
            }
        }
    });

    audio_thread.join().unwrap();
    user_input_thread.join().unwrap();
}
