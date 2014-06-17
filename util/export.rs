use std::io::{File, Truncate, Write};
use raytracer::compositor::Surface;
use raytracer::compositor::colorrgba;

#[allow(unused_must_use)]
pub fn to_ppm(surface: Surface, filename: &str) -> () {
    let header = format!("P3 {} {} {}\n", surface.width, surface.height, colorrgba::consts::MAX_COLOR);

    let path = Path::new(filename);
    let mut f = match File::open_mode(&path, Truncate, Write) {
        Ok(f)  => {f},
        Err(e) => fail!("File error: {}", e),
    };

    f.write(header.as_bytes());
    for pixel in surface.buffer.iter() {
        f.write(format!("{} {} {} ", pixel.r, pixel.g, pixel.b).as_bytes());
    }
}
