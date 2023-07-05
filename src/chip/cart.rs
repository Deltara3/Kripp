use std::fs::File;
use std::io::{Read, BufReader};
use std::path::PathBuf;

pub fn read(location: PathBuf) -> Vec<u8> {
    let file = File::open(location).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    return buffer;
}
