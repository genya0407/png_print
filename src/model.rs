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

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}
impl Image {
    pub fn scanlines(&self) -> Vec<Vec<Color>> {
        let effective_width = self.pixels.len() / self.height;
        let mut reversed_pixels = self.pixels.clone();
        reversed_pixels.reverse();

        let mut reversed_scanlines: Vec<Vec<Color>> = Vec::new();
        for scanline_nth in 0..self.height {
            let scanline_start      = scanline_nth * effective_width;
            let next_scanline_start = scanline_start + effective_width;
            let mut scanline = reversed_pixels[scanline_start..next_scanline_start].to_vec();
            reversed_scanlines.push(scanline);
        }
        reversed_scanlines
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
    pub fn height(&self) -> usize {
        self.ihdr.height as usize
    }

    pub fn width(&self) -> usize {
        self.ihdr.width as usize
    }

    pub fn decompress(&self) -> Result<Vec<u8>, String> {
        let mut decompressed_data = Vec::new();
        for idat in self.idats.iter() {
            decompressed_data.extend(inflate_bytes(&idat.compressed_data)?);
        }
        Ok(decompressed_data)
    }

    fn decompress_with_color(&self) -> Result<Vec<Color>, String> {
        let decompressed_bits = self.decompress()?;
        match self.plte_opt {
            Some(ref plte) => {
                let mut colors: Vec<Color> = Vec::new();
                for decompressed_bit in decompressed_bits {
                    colors.push(plte.colors[decompressed_bit as usize].clone());
                }
                Ok(colors)
            },
            None => {
                let data_length = decompressed_bits.len();
                let scanline_width = data_length / self.height();
                let mut colors = Vec::new();
                for scanline_nth in 0..self.height() {
                    let scanline_start_at = scanline_nth * scanline_width;
                    let next_scanline_start_at = (scanline_nth + 1) * scanline_width;
                    let scanline = &decompressed_bits[scanline_start_at..next_scanline_start_at];
                    let color_part_of_scanline = &scanline[1..];
                    let filter_method = scanline[0];
                    let colors_of_scanline = match filter_method {
                        0 => Color::new_vector(color_part_of_scanline, 4),
                        1 => Color::with_sub(color_part_of_scanline),
                        _ => return Err("not implemented filter method!".to_string()),
                    };
                    colors.extend(colors_of_scanline);
                }
                Ok(colors)
            }
        }
    }

    pub fn to_image(&self) -> Result<Image, String> {
        let pixels = self.decompress_with_color()?;
        let image = Image { width: self.ihdr.width as usize, height: self.ihdr.height as usize, pixels: pixels };
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
    pub alpha: u8,
}
impl Color {
    pub fn new_vector(flatten_slice: &[u8], step: usize) -> Vec<Self> {
        let mut colors = Vec::new();
        for base_index in 0..(flatten_slice.len() / step) {
            let color = Self {
                red:   flatten_slice[base_index*step],
                green: flatten_slice[base_index*step+1],
                blue:  flatten_slice[base_index*step+2],
                alpha: flatten_slice[base_index*step+3],
            };
            colors.push(color);
        }
        colors
    }

    pub fn with_sub(scanline: &[u8]) -> Vec<Self> {
        let bpp = 4;
        let mut raws = Vec::new();
        for x in 0..scanline.len() {
            let sub_x = scanline[x];
            let raw_x_bpp = if x < bpp { 0 } else { raws[x - bpp] };
            let raw_x: u8 = sub_x.wrapping_add(raw_x_bpp);
            raws.push(raw_x);
        }
        Self::new_vector(&raws, 4)
    }
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
        Self { colors: Color::new_vector(chunk_data, 3) }
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

