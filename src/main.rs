mod ppm;
use ppm::PPMWriter;

fn main() -> std::io::Result<()> {
    let height = 720;
    let width = 1080;
    let ppm_writer = PPMWriter::create(width, height, "out.ppm");
    ppm_writer.write()
}
