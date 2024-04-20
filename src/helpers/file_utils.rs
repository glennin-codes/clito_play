use std::path::Path;

pub fn get_file_name(file_path: &Path)->Option<&str>{
    let path=Path::new(file_path);
    path.file_stem()?.to_str()

}