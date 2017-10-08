use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env::args;
use std::str;

fn main() {
    let filename = args().nth(1).unwrap();
    let bytes = readfile(&filename).unwrap();
    let chunks = parse_to_chunks(bytes);
    for c in chunks.unwrap() {
        println!("{}", c.chunk_type());
    }
}

trait IChunk {
    //fn chunk_length(&self) -> u32;
    fn chunk_type(&self) -> String;
    //fn chunk_data(&self) -> &Vec<u8>;
    //fn chunk_crc(&self) -> u32;
}

#[derive(Debug)]
struct GeneralChunk {
    chunk_length: u32,
    chunk_type: String,
    chunk_data: Vec<u8>,
    chunk_crc: [u8; 4],
}

struct 

impl IChunk for GeneralChunk {
    fn chunk_type(&self) -> String {
        self.chunk_type.clone()
    }
}

const PNG_HEADER_SIZE: usize = 8;
fn parse_to_chunks(bytes: Vec<u8>) -> Result<Vec<Box<IChunk>>, Box<Error>> {
    let mut chunks: Vec<Box<IChunk>> = Vec::new();
    let mut iter = bytes.into_iter().skip(PNG_HEADER_SIZE);
    loop {
        let chunk_length = (iter.next().unwrap() as u32) * (2u32.pow(24)) +
            (iter.next().unwrap() as u32) * (2u32.pow(16)) +
            (iter.next().unwrap() as u32) * (2u32.pow(8)) +
            (iter.next().unwrap() as u32);
        let chunk_type = format!("{}{}{}{}",
                                 iter.next().unwrap() as char,
                                 iter.next().unwrap() as char,
                                 iter.next().unwrap() as char,
                                 iter.next().unwrap() as char);
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

        let chunk = GeneralChunk {
            chunk_length: chunk_length,
            chunk_type: chunk_type,
            chunk_data: chunk_data,
            chunk_crc: chunk_crc,
        };
        if chunk.chunk_type() == "IEND".to_string() {
            chunks.push(Box::new(chunk));
            break;
        } else {
            chunks.push(Box::new(chunk));
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
