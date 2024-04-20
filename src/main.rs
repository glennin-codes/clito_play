use clito_play::helpers::get_directories::directory;
#[allow(unused_assignments)]
use clito_play::{controller::get_audio_files::all_listed_audio_files, controller::play::play_audios};


// const CHUNK_SIZE: usize = 9568; // Set the chunk size to 4096 bytes (4 KB)

fn main() {
    println!("Welcome to CLIToPlay, play your music on the terminal");
    println!("scanning files for audio please wait ...");
    //scan and get all audio files from your files system
    let audio_files = all_listed_audio_files(&directory());
    println!("Below are all your music audio files; ");
if audio_files.len() < 1{
   println!( "No Audio Files Found in this directory.");
   return;
}else{
    // Display the found audio files and prompt the user to select files
    for (i, file) in audio_files.iter().enumerate() {
        println!("{}: {}", i + 1, file.display());
    }
    println!(
        "Enter the indices of the music you want to play as your playlist (e.g., 1,3,5), or 'all' to play all files:"
    );

   
 //play each song in the playlist;
    play_audios();
    println!("type  `n` then enter  to play the next song ");
    println!("type `p` then enter to play the previous song ");
    println!("type `q` then enter to quit the stream");
    // println!("type `r` to repeat your playlist");
    println!("Note:repeat is set to true by default after end of the  songs press q");
    println!("type `v+ OR v- OR v` for volume control");
    println!("type` f+ OR f- Or F` to forward & rewind control");
    println!("type 'help' to see available commands");
}
    
}
