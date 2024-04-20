use clito_play::helpers::get_directories::directory;
#[allow(unused_assignments)]
use clito_play::{controller::get_audio_files::all_listed_audio_files, controller::play::play_audios};


// const CHUNK_SIZE: usize = 9568; // Set the chunk size to 4096 bytes (4 KB)

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

   
 //play each song in the playlist;
    play_audios();
    println!("playing music .. ");
    println!("type  `n` then enter  to play the next song ");
    println!("type `p` then enter to play the previous song ");
    println!("type `q` then enter to quit the stream");
    println!("type `r` to repeat your playlist");
    println!("Note:repeat set to true by default");
    println!("type` s` to forward");
   
    
}
