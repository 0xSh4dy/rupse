use std::fs::File;
use std::io::{Error, Read};

pub fn open_file(file_path: String) -> Result<File, Error> {
    File::open(file_path)
}

pub fn read_file(mut file: File) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer);
    return buffer;
}
