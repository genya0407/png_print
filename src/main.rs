extern crate byteorder;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env::args;
use std::str;
use byteorder::{BigEndian, ReadBytesExt};

mod model;
use model::*;

fn main() {
    let filename = args().nth(1).unwrap();
    let bytes = readfile(&filename).unwrap();
    let chunks = parse_to_chunks(bytes);
    for chunk in chunks.unwrap().into_iter() {
        println!("{:?}", chunk.chunk_type);
    }
}

//fn parse_to_png(chunks: Vec<GeneralChunk>) -> Result<PNG, Box<Error>> {
//}

const PNG_HEADER_SIZE: usize = 8;
fn parse_to_chunks(bytes: Vec<u8>) -> Result<Vec<GeneralChunk>, Box<Error>> {
    let mut chunks = Vec::new();
    let mut base_index: usize = PNG_HEADER_SIZE;
    loop {
        let chunk_length = (&bytes[base_index..base_index+4]).read_u32::<BigEndian>()? as usize;
        let chunk_type = String::from(str::from_utf8(&bytes[base_index+4..base_index+8])?);
        let chunk_data_slice = &bytes[(base_index+8)..(base_index+8+chunk_length)];
        let chunk_crc = (&bytes[(base_index+chunk_length)..(base_index+chunk_length+4)]).read_u32::<BigEndian>()?;

        let mut chunk_data: Vec<u8> = Vec::new();
        chunk_data.extend_from_slice(chunk_data_slice);
        let chunk = GeneralChunk {
            chunk_length: chunk_length,
            chunk_type: chunk_type,
            chunk_data: chunk_data,
            chunk_crc: chunk_crc,
        };
        base_index += 12 + chunk_length;

        if chunk.chunk_type == "IEND".to_string() {
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
