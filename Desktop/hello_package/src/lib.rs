
pub mod controller;
pub mod directories{
    use std::{env, path::PathBuf};

   pub fn directory()->PathBuf{
        let args: Vec<String> = env::args().collect();();

        let search_dir = if args.len() > 1 {
            PathBuf::from(&args[1])
        } else {
            env::current_dir().unwrap()
        };
     
        search_dir
    }
}