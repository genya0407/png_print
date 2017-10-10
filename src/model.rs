use byteorder::{BigEndian, ReadBytesExt};
use std::error;
use std::fmt;
use inflate::inflate_bytes;

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

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}
impl Image {
    pub fn half_half(&self) -> Self {
        let n = 2;

        let mut new_pixels: Vec<Color> = Vec::new();
        for h in (0..self.height).filter(|i| i % n == 0) {
            for w in (0..self.width).filter(|i| i % n == 0) {
                let color = &self.pixels[(h * self.width + w) as usize];
                new_pixels.push(color.clone());
            }
        }

        let new_width = self.width / n;
        let new_height = self.height / n;

        Self {
            width: new_width,
            height: new_height,
            pixels: new_pixels,
        }
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
impl Png {
    fn decompress(&self) -> Result<Vec<u8>, String> {
        let mut concatenated_data = Vec::new();
        for idat in self.idats.iter() {
            concatenated_data.extend(idat.compressed_data.clone());
        }
        inflate_bytes(concatenated_data.as_slice())
    }
    
    fn decompress_with_color(&self) -> Result<Vec<Color>, String> {
        let plte = self.plte_opt.clone().ok_or("PLTE chunk doesn't exist!")?;
        let decompressed_bits = self.decompress()?;
        let mut colors: Vec<Color> = Vec::new();
        for decompressed_bit in decompressed_bits {
            colors.push(plte.colors[decompressed_bit as usize].clone());
        }
        Ok(colors)
    }

    pub fn to_image(&self) -> Result<Image, String> {
        let pixels = self.decompress_with_color()?;
        let image = Image { width: self.ihdr.width, height: self.ihdr.height, pixels: pixels };
        Ok(image)
    }
}

#[derive(Debug,Clone)]
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

#[derive(Debug, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
#[derive(Clone)]
pub struct Plte {
    pub colors: Vec<Color>
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
            let color = Color {
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
    compression_method: u8,
    additional_flags: u8,
    compressed_data: Vec<u8>,
    check_value: u32,
}
impl fmt::Debug for Idat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IDAT: {} bytes.", self.compressed_data.len())
    }
}
impl Idat {
    pub fn new(data_bytes: Vec<u8>) -> Self {
        let check_value_start_at = data_bytes.len() - 4;

        Self {
            compression_method: data_bytes[0],
            additional_flags: data_bytes[1],
            compressed_data: (&data_bytes[2..check_value_start_at]).to_vec(),
            check_value: (&data_bytes[check_value_start_at..]).read_u32::<BigEndian>().unwrap(),
        }
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

