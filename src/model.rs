use byteorder::{BigEndian, ReadBytesExt};
use std::error::Error;

pub struct PNG {
    pub ihdr: IHDR,
    pub plte: PLTE,
    pub idats: Vec<IDAT>,
    pub iend: IEND,
    pub others: Vec<GeneralChunk>
}

pub struct IDAT {}
pub struct IEND {}
pub struct PLTE {}

#[derive(Debug)]
pub struct GeneralChunk {
    pub chunk_length: usize,
    pub chunk_type: String,
    pub chunk_data: Vec<u8>,
    pub chunk_crc: u32,
}
impl GeneralChunk {
    pub fn to_ihdr(self) -> IHDR {
        IHDR::new(self.chunk_length, &self.chunk_data, self.chunk_crc)
    }

    pub fn to_idat(self) -> IDAT {
        IDAT {}
    }

    pub fn to_iend(self) -> IEND {
        IEND {}
    }

    pub fn to_plte(self) -> PLTE {
        PLTE {}
    }
}

#[derive(Debug)]
pub struct IHDR {
    pub chunk_length:       usize,
    pub chunk_crc:          u32,
    pub width:              u32,
    pub height:             u32,
    pub bit_depth:          u8,
    pub color_type:         u8,
    pub compression_method: u8,
    pub filter_method:      u8,
    pub interlace_method:   u8,
}
impl IHDR {
    pub fn new(chunk_length: usize, chunk_data: &[u8], chunk_crc: u32) -> Self {
        Self {
            chunk_length: chunk_length,
            chunk_crc: chunk_crc,
            width: (&chunk_data[0..4]).read_u32::<BigEndian>().unwrap(),
            height: (&chunk_data[4..8]).read_u32::<BigEndian>().unwrap(),
            bit_depth: chunk_data[8],
            color_type: chunk_data[9],
            compression_method: chunk_data[10],
            filter_method: chunk_data[11],
            interlace_method: chunk_data[12],
        }
    }
}
