use std::fs::{remove_file, File};
use std::path::PathBuf;

pub fn force_create(path: PathBuf) -> File {
    if path.exists() {
        remove_file(path.to_owned()).unwrap();
    }
    File::create(path.to_owned()).unwrap()
}
