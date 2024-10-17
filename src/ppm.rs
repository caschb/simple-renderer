use std::fs::File;
use std::io::Write;
use std::vec;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    fn to_string(&self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
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
        let buffer = vec![Color::GREEN; buf_size as usize];
        PPMWriter {
            width,
            height,
            buffer,
            filename,
        }
    }

    pub fn draw_pixel(&mut self, i: u32, j: u32, color: Color) {
        let idx = (j * self.width + i) as usize;
        println!("Hello!\n");
        self.buffer[idx] = color;
    }

    pub fn write(&self) -> std::io::Result<()> {
        let mut f = File::create(self.filename)?;
        let color_strings = self
            .buffer
            .iter()
            .map(|color| color.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        f.write(
            format!(
                "P3\n{} {}\n255\n{}\n",
                self.width, self.height, color_strings
            )
            .as_bytes(),
        )?;
        Ok(())
    }
}
