use std::old_io::{File, Truncate, Write};
use raytracer::compositor::{Surface, Channel};

#[allow(unused_must_use)]
pub fn to_ppm(surface: Surface, filename: &str) {
    let channel_max: u8 = Channel::max_value();
    let header = format!(
        "P3 {} {} {}\n", surface.width, surface.height,
        channel_max);

    let path = Path::new(filename);
    let mut f = match File::open_mode(&path, Truncate, Write) {
        Ok(f)  => f,
        Err(e) => panic!("File error: {}", e),
    };

    f.write_all(header.as_bytes());
    for pixel in surface.buffer.iter() {
        f.write_all(format!("{} {} {} ", pixel.r, pixel.g, pixel.b).as_bytes());
    }
}
