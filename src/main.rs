#[allow(unused_assignments)]
use hello_package::{ controller::playlist::create_playlist, directories::directory };
use rodio::{ Decoder, OutputStream, Sink };
use std::{ fs::File, io::{stdin, BufRead, BufReader}, path::PathBuf, sync::Condvar, thread, time::Duration };
use hello_package::controller::get_audio_files::all_listed_audio_files;
use std::sync::{ Arc, Mutex };

const CHUNK_SIZE: usize = 9568; // Set the chunk size to 4096 bytes (4 KB)

fn main() {
    println!("Welcome to CLIToPlay, play your music on the terminal");
    println!("Below are all your music audio files; ");

    //scan and get all audio files from your files system
    let audio_files = all_listed_audio_files(&directory());
    // Display the found audio files and prompt the user to select files
    for (i, file) in audio_files.iter().enumerate() {
        println!("{}: {}", i + 1, file.display());
    }
    println!(
        "Enter the indices of the music you want to play as your playlist (e.g., 1,3,5), or 'all' to play all files:"
    );

    //use the above prompt to creat your  playlists
    let mut playlist = create_playlist(audio_files);

    //start streaming
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
    let mut playlist = Arc::new(Mutex::new(playlist));
    let sink_clone = Arc::clone(&sink);
    let playlist_clone = Arc::clone(&playlist);
    let condvar = Arc::new(Condvar::new());
    let condvar_clone = Arc::clone(&condvar);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
    println!("type  `n` then enter  to play the next song ");
    println!("type `p` then enter to play the previous song ");
    println!("type `q` then enter to quit the stream");
    println!("playing music .. ");
    let mut prev_file_path: Option<PathBuf> = None;
    // Spawn a thread to check if the song has ended
  
    while !playlist.lock().unwrap().is_empty() {
        let file_path = playlist.lock().unwrap().pop_front().unwrap();

        let file = File::open(&file_path).unwrap();

        // Use BufReader with larger buffer size
        let mut buf_reader = BufReader::with_capacity(CHUNK_SIZE, file);
        let source = Decoder::new(buf_reader).unwrap();

        sink.lock().unwrap().append(source);

        while sink.lock().unwrap().empty() == false {
             // Check if the playback has finished
        if sink.lock().unwrap().empty() {
            println!("Song ended, moving to the next song...");
            // Ensure the loop continues to the next song
            continue;
        }
            let stdin = stdin();
            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();

            match input.trim() {
                "k" => {
                    if sink.lock().unwrap().is_paused() {
                        println!("Resumed");
                        sink.lock().unwrap().play();
                    } else {
                        println!("Paused");
                        sink.lock().unwrap().pause();
                    }
                }
                "n" => {
                    println!("Clicked next");
                    sink.lock().unwrap().stop();
                    break; // Break out of the loop to proceed to the next song
                }
                "p" => {
                    println!("Clicked prev");
                    // Implement logic to move to the previous song
                    sink.lock().unwrap().stop();
                    break; // Break out of the loop to proceed to the previous song
                }
                "s" => {
                    let speed = sink.lock().unwrap().speed();
                    if speed == 1.0 {
                        sink.lock().unwrap().set_speed(4.0);
                    } else {
                        sink.lock().unwrap().set_speed(1.0);
                    }
                }
                "q" => {
                    println!("Quitted...");
                    return;
                }

                _ => {
                    println!(
                        "Invalid input. Please enter 'n' to play the next song, 'p' to play the previous song, 'k' to pause/resume, or 'q' to quit."
                    );
                }
            }
        }
       
    }
    thread::spawn(move || {
        while !playlist_clone.lock().unwrap().is_empty() {
            let mut sink_guard = sink_clone.lock().unwrap();
            while !sink_guard.empty() {
                sink_guard = condvar_clone.wait(sink_guard).unwrap();
            }
            println!("Song ended in fast thread, moving to the next song...");
            // Implement logic to move to the next song
            break; // Exit the loop when the song ends
        }
    });
    
    // Check if the playlist is empty after the loop
    if playlist.lock().unwrap().is_empty() {
        println!("All songs have been played. Press 'j' to repeat all songs or 'q' to quit.");
        // Handle user input for repeating or quitting
    }
}
