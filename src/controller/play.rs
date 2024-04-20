use std::fs::File;
use std::path::Path;
use std::sync::{ Arc, Mutex, Condvar };
use std::thread::{ self };
use std::io::{ stdin, BufRead };
use std::time::Duration;
use rodio::{ Decoder, OutputStream, Sink };

use crate::helpers::file_utils::get_file_name;

use super::get_audio_files::all_listed_audio_files;
use super::playlist::create_playlist;
use crate::helpers::get_directories::directory;

pub fn play_audios() {
    //scan and get all audio files from your files system
    let audio_files = all_listed_audio_files(&directory());
    //list of playlist chosen by the user
    let list = create_playlist(audio_files);
    if list.len()<1{
        return;
    }
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
    let restart_playlist = Arc::new(Mutex::new(true));
    let restart_condvar = Arc::new(Condvar::new());
    let restart_condvar_clone = Arc::clone(&restart_condvar);
    let restart_playlist_clone = Arc::clone(&restart_playlist);

    let audio_thread = thread::spawn(move || {
        loop {
            let mut restart_playlist = restart_playlist_clone.lock().unwrap();
            let mut timeout_expired = false;

            while *restart_playlist == false {
                let (result, wait_time_out_result) = restart_condvar_clone
                    .wait_timeout(restart_playlist, Duration::from_secs(20))
                    .unwrap();
                restart_playlist = result;
                restart_condvar_clone.notify_one();
                if wait_time_out_result.timed_out() {
                    timeout_expired = true;
                    break;
                }
            }
            if timeout_expired {
                // If the timeout expires without the condition being met, exit the thread or program
                println!(
                    "No action taken within 20 seconds. Please Rerun your program by providing the file path for where your songs are."
                );
            return;
            } else {
                loop {
                    let playlist = playlist_clone.lock().unwrap();
                    let mut current_song = *current_song_clone.lock().unwrap();
                    if current_song < playlist.len() {
                        let file_path = Path::new(&playlist[current_song]);

                        if let Some(file_name) = get_file_name(file_path) {
                            println!("now playing .. {}. {}", current_song + 1, file_name);
                        }

                        //playing the song
                        let source = Decoder::new(
                            File::open(&playlist[current_song]).unwrap()
                        ).unwrap();

                        sink.append(source);
                        // sink.sleep_until_end();

                        while sink.empty() == false {
                            let mut current_song_guard = current_song_clone.lock().unwrap();
                            if *current_song_guard != current_song {
                                println!("executed on play");
                                current_song = *current_song_guard;
                                println!(" guard :{}", current_song_guard);
                                println!(" curent song :{}", current_song);

                                break;
                            }
                        }
                        while sink.empty() == true {
                            println!("Song ended, moving to the next song...");

                            println!("why not executed");
                            current_song += 1;
                            condvar_clone.notify_one();
                            let mut current_song_guard = current_song_clone.lock().unwrap();
                            if *current_song_guard < current_song {
                                *current_song_guard = current_song;
                                println!("executed on play");
                                current_song = *current_song_guard;
                                println!(" guard :{}", current_song_guard);
                                println!(" curent song :{}", current_song);

                                break;
                            }

                            println!("{}", current_song);
                        }
                    } else {
                        println!("executed  else part of loop");
                        println!("restart playlist {}", restart_playlist);
                        let mut current_song_guard = current_song_clone.lock().unwrap();

                        *current_song_guard = 0;
                        current_song = *current_song_guard;
                        println!(" guard :{}", current_song_guard);
                        println!(" curent song :{}", current_song);
                        if !*restart_playlist {
                            println!(
                                "Your playlist queue has completed. Press 'r' to repeat, 'q' to quit."
                            );
                            break;
                        } else {
                            println!("playlist complete! repeating playlist... press q to quit!");
                            break;
                        }
                    }
                }
            }
        }
    });

    let user_input_thread = thread::spawn(move || {
        let mut speed: f32 = 1.0;
        let mut volume: f32 = 1.0;
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
                        sink_clone.stop();
                        *current_song -= 1;
                    }
                    condvar.notify_one(); // Notify the audio thread of the change
                }
                // "r"=>{
                //     println!("r clicked ");
                //     let mut restart_playlist_guard = restart_playlist.lock().unwrap(); // Lock the mutex and get a mutable reference to the data
                //     *restart_playlist_guard = !*restart_playlist_guard; // Toggle the flag
                //     println!("restart:{}", *restart_playlist_guard); // Print the current state
                //     restart_condvar.notify_one(); // Notify the waiting thread
                // }
                
                
                "k" => {
                    if sink_clone.is_paused() {
                        println!("Resumed");
                        sink_clone.play();
                    } else {
                        println!("Paused");
                        sink_clone.pause();
                    }
                }
                "v" => {
                    // Return to default volume
                    volume = 1.0;
                    sink_clone.set_volume(volume);
                }
                "v+" => {
                    // Increase volume
                    volume = (volume * 2.0).min(2.0); // Ensure volume doesn't exceed 2.0
                    sink_clone.set_volume(volume);
                }
                "v-" => {
                    // Decrease volume
                    volume = (volume / 2.0).max(0.0); // Ensure volume doesn't go below 0.0
                    sink_clone.set_volume(volume);
                }
                
                "f" => {
                    // Return to default speed
                    speed = 1.0;
                    sink_clone.set_speed(speed);
                }
                "f+" => {
                    // Increase speed
                    speed *= 2.0;
                    sink_clone.set_speed(speed);
                    println!("Speed is now {}", speed);
                }
                "f-" => {
                    // Decrease speed
                    speed /= 2.0;
                    sink_clone.set_speed(speed);
                    println!("Speed is now {}", speed);
                    println!("\n");
                }
                "fmax" => {
                    // Set to maximum speed supported
                    speed = f32::INFINITY;
                    sink_clone.set_speed(speed);
                    println!("Maximum playback speed reached.");
                }
                "help" => {
                    println!("Available commands:");
                    println!("f    : Return to default speed");
                    println!("f+   : Increase speed");
                    println!("f-   : Decrease speed");
                    println!("v    : Return to default volume");
                    println!("v+   : Increase volume by 0.5");
                    println!("v-   : Decrease volume by 0.5");
                    println!("n    : Play next song");
                    println!("p    : Play previous song");
                    println!("k    : Toggle play/pause");
                    println!("r    : Toggle repeat playlist");
                    println!("fmax : Set to maximum speed supported");
                    println!("help : Show available commands");
                    println!("q    : Quit");
                }
                "q" => {
                    println!("are you sure you want to quit");
                    let stdn = std::io::stdin();
                    let mut input = String::new();
                    stdn.lock().read_line(&mut input).unwrap();
                    if input.trim() == "y" {
                        println!("Quitting...");

                        condvar.notify_one();
                        return;
                    }
                }
                _ =>
                    println!(
                        "invalid command {} .type 'help' to see available commands'",
                        line.as_str()
                    ),
            }
        }
    });

    audio_thread.join().unwrap();
    user_input_thread.join().unwrap();
}
