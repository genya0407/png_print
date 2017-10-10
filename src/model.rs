use byteorder::{BigEndian, ReadBytesExt};
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidPngFileError {
    pub lacking_chunk_type: String,
}
impl InvalidPngFileError {
    pub fn new(lacking_chunk_type: String) -> Self {
        Self { lacking_chunk_type: lacking_chunk_type }
    }
}
impl fmt::Display for InvalidPngFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid png file. {}", self.lacking_chunk_type)
    }
}
impl error::Error for InvalidPngFileError {
    fn description(&self) -> &str {
        "Invalid png file."
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
pub struct Png {
    pub ihdr: Ihdr,
    pub plte_opt: Option<Plte>,
    pub idats: Vec<Idat>,
    pub iend: Iend,
    pub others: Vec<GeneralChunk>
}

#[derive(Debug)]
pub struct Ihdr {
    pub width:              u32,
    pub height:             u32,
    pub bit_depth:          u8,
    pub color_type:         u8,
    pub compression_method: u8,
    pub filter_method:      u8,
    pub interlace_method:   u8,
}
impl Ihdr {
    pub fn new(chunk_data: &[u8]) -> Self {
        Self {
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

#[derive(Debug)]
pub struct PlteColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
pub struct Plte {
    pub colors: Vec<PlteColor>
}
impl fmt::Debug for Plte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PLTE: {} colors.", self.colors.len())
    }
}
impl Plte {
    pub fn new(chunk_data: &[u8]) -> Self {
        let mut colors = Vec::new();
        for base_index in 0..(chunk_data.len() / 3) {
            let color = PlteColor {
                red:   chunk_data[base_index*3],
                green: chunk_data[base_index*3+1],
                blue:  chunk_data[base_index*3+2],
            };
            colors.push(color);
        }
        Self { colors: colors }
    }
}

pub struct Idat {
    data: Vec<u8>,
}
impl fmt::Debug for Idat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IDAT: {} bytes.", self.data.len())
    }
}
impl Idat {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data: data }
    }
}
#[derive(Debug)]
pub struct Iend {}
impl Iend {
    pub fn new() -> Self {
        Iend {}
    }
}

#[derive(Debug)]
pub struct GeneralChunk {
    pub chunk_length: usize,
    pub chunk_type: String,
    pub chunk_data: Vec<u8>,
    pub chunk_crc: u32,
}
impl GeneralChunk {
    pub fn to_ihdr(self) -> Ihdr {
        Ihdr::new(&self.chunk_data)
    }

    pub fn to_idat(self) -> Idat {
        Idat::new(self.chunk_data)
    }

    pub fn to_iend(self) -> Iend {
        Iend::new()
    }

    pub fn to_plte(self) -> Plte {
        Plte::new(&self.chunk_data)
    }
}

