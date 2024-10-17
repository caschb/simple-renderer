mod ppm;
use ppm::{Color, PPMWriter};

fn main() -> std::io::Result<()> {
    let height = 720;
    let width = 1080;
    let mut ppm_writer = PPMWriter::create(width, height, "out.ppm");
    ppm_writer.draw_pixel(width / 2, height / 2, Color::RED);
    ppm_writer.write()
}
