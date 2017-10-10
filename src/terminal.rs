use ansi_term::Colour::RGB;
use model::*;
use std::io::{stdout, Write, BufWriter};

pub fn show_on_terminal(png: Png) -> Result<(), String> {
    let height = png.ihdr.height;
    let width = png.ihdr.width;
    let colors = png.decompress_with_color()?;
    let mut image = String::new();
    for h in 0..height {
        for w in 0..width {
            let color = &colors[(h * width + w) as usize];
            image += &format!("{}", RGB(color.red(), color.green(), color.blue()).paint("■"));
        }
        image += "\n";
    }
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    out.write_all(image.as_bytes()).unwrap();
    Ok(())
}
