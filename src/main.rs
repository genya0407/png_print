use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env::args;
use std::str;

fn main() {
    let filename = args().nth(1).unwrap();
    let bytes = readfile(&filename).unwrap();
    let chunks = get_chunks(bytes);
    for c in chunks.unwrap() {
        println!("{}", c.chunk_type_string());
    }
}

#[derive(Debug)]
struct Chunk {
    pub chunk_length: u32,
    pub chunk_type: [u8; 4],
    pub chunk_data: Vec<u8>,
    pub chunk_crc: [u8; 4],
}

impl Chunk {
    pub fn chunk_type_string(&self) -> String {
        format!("{}{}{}{}", self.chunk_type[0] as char, self.chunk_type[1] as char, self.chunk_type[2] as char, self.chunk_type[3] as char)
    }
}

const PNG_HEADER_SIZE: usize = 8;
fn get_chunks(bytes: Vec<u8>) -> Result<Vec<Chunk>, Box<Error>> {
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut iter = bytes.into_iter().skip(PNG_HEADER_SIZE);
    loop {
        let chunk_length = (iter.next().unwrap() as u32) * (2u32.pow(24)) +
            (iter.next().unwrap() as u32) * (2u32.pow(16)) +
            (iter.next().unwrap() as u32) * (2u32.pow(8)) +
            (iter.next().unwrap() as u32);
        let chunk_type = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap()
        ];
        let mut chunk_data = Vec::new();
        for _ in 0..chunk_length {
            chunk_data.push(iter.next().unwrap());
        }
        let chunk_crc = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap()
        ];

        let chunk = Chunk {
            chunk_length: chunk_length,
            chunk_type: chunk_type,
            chunk_data: chunk_data,
            chunk_crc: chunk_crc,
        };
        if chunk.chunk_type_string() == "IEND".to_string() {
            chunks.push(chunk);
            break;
        } else {
            chunks.push(chunk);
        }
    }
    Ok(chunks)
}

fn readfile(filename: &str) -> Result<Vec<u8>, Box<Error>> {
    let path = Path::new(filename);
    let mut file = File::open(&path)?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;
    Ok(buf)
}
