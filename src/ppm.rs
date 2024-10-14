use std::fs::File;
use std::io::Write;
use std::vec;

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    const RED: Color = Color { r: 255, g: 0, b: 0 };
    const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    const BLUE: Color = Color { r: 0, g: 0, b: 255 };
    const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
}

pub struct PPMWriter {
    width: u32,
    height: u32,
    buffer: Vec<Color>,
    filename: &'static str,
}

impl PPMWriter {
    pub fn create(width: u32, height: u32, filename: &'static str) -> PPMWriter {
        let buf_size = width * height;
        let buffer = vec![Color::BLACK; buf_size as usize];
        PPMWriter {
            width,
            height,
            buffer,
            filename,
        }
    }
    pub fn write(&self) -> std::io::Result<()> {
        let mut f = File::create(self.filename)?;
        f.write("P3\n".as_bytes())?;
        f.write(self.width.to_string().as_bytes())?;
        f.write(" ".as_bytes())?;
        f.write(self.height.to_string().as_bytes())?;
        f.write("\n255\n".as_bytes())?;
        for color in self.buffer.iter() {
            f.write(color.r.to_string().as_bytes())?;
            f.write(" ".as_bytes())?;
            f.write(color.g.to_string().as_bytes())?;
            f.write(" ".as_bytes())?;
            f.write(color.b.to_string().as_bytes())?;
            f.write("\n".as_bytes())?;
        }
        Ok(())
    }
}
