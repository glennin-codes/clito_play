use hello_package::controller::playlist:: create_playlist;
use rodio::{ Decoder, OutputStream, Sink };
use std::{fs::File, io::BufReader};
use hello_package::controller::get_audio_files::all_listed_audio_files;
#[allow(unused_assignments)]
fn main() {
    //scan and get all audio files from your files system
 let audio_files=all_listed_audio_files();
    // Display the found audio files and prompt the user to select files
    for (i, file) in audio_files.iter().enumerate() {
        println!("{}: {}", i + 1, file.display());
    }
    println!(
        "Enter the indices of the files you want to play (e.g., 1,3,5), or 'all' to play all files:"
    );

//use the above prompt to creat your  playlists 
    let mut playlist=create_playlist(audio_files);
  
    println!("{:?}", playlist);
    //start streaming
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

   

    let mut prev_file_path = None;

    while !playlist.is_empty() {
        let file_path = playlist.pop_front().unwrap();
        let file = File::open(&file_path).unwrap();
        let buf_reader = BufReader::new(file);
        let source = Decoder::new(buf_reader).unwrap();

        sink.append(source);

        while sink.empty() == false {
            let stdin = std::io::stdin();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            println!("clicked {}",input);
            if input.trim() == " " {
                println!("paused");
                sink.is_paused();
            } else if input.trim() == "n" {
                println!("clicked next leter n");
                sink.stop();
                prev_file_path = Some(file_path);
                break;
            } else if input.trim() == "p" {
                println!("clicked previos leter p");
                if let Some(prev_path) = prev_file_path.take() {
                    playlist.push_front(prev_path);
                }
                sink.stop();
                break;
            } else if  input.trim() == "q"{
                println!("Quitting...");
                return;
            }else{
                
            }
                
                
             
           
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
