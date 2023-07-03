use std::fs::File;
use std::io::{Read, BufReader};

pub fn read(location: &str) -> Vec<u8> {
    let file = File::open(location).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    return buffer;
}
