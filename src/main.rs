use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env::args;

fn main() {
    let filename = args().nth(1).unwrap();
    let bytes = readfile(&filename).unwrap();
}

struct Chunk {
    pub chunk_length: [u8; 4],
    pub chunk_type: [u8; 4],
    pub chunk_data: Vec<u8>,
    pub chunk_crc: [u8; 4],
}

fn split_to_chunks(buf: Vec<u8>) -> Vec<Chunk> {
    vec![
        Chunk {
            chunk_length: [1,1,1,1],
            chunk_type: [1,1,1,1],
            chunk_data: Vec::new(),
            chunk_crc: [1,1,1,1],
        }
    ]
}

fn readfile(filename: &str) -> Result<Vec<u8>, Box<Error>> {
    let path = Path::new(filename);
    let mut file = File::open(&path)?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;
    Ok(buf)
}
