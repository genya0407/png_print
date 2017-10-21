use ansi_term::Colour::RGB;
use model::*;
use std::io::{stdout, Write, BufWriter};

pub fn show_on_terminal(image: Image) {
    let mut terminal_image = String::new();
    for scanline in image.scanlines() {
        for color in scanline {
            terminal_image += &format!("{}", RGB(color.red, color.green, color.blue).paint("■"));
        }
        terminal_image += "\n";
    }
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    out.write_all(terminal_image.as_bytes()).unwrap();
}
