use ansi_term::Colour::RGB;
use model::*;
use std::io::{stdout, Write, BufWriter};

pub fn show_on_terminal(image: Image) {
    let mut terminal_image = String::new();
    for h in 0..image.height {
        for w in 0..image.width {
            let color = &image.pixels[(h * image.width + w) as usize];
            terminal_image += &format!("{}", RGB(color.red, color.green, color.blue).paint("■"));
        }
        terminal_image += "\n";
    }
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    out.write_all(terminal_image.as_bytes()).unwrap();
}
