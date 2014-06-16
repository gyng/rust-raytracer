use std::io::{File, Truncate, Write};
use std::cmp::{min, max};
use vec3::Vec3;

#[allow(unused_must_use)]
pub fn to_ppm(image_data: Vec<Vec3>, width: int, height: int, filename: &str) -> () {
    let min_color = 0;
    let max_color = 255;
    let header = format!("P3 {} {} {}\n", width, height, max_color);

    let path = Path::new(filename);
    let mut f = match File::open_mode(&path, Truncate, Write) {
        Ok(f)  => {f},
        Err(e) => fail!("File error: {}", e),
    };

    f.write(header.as_bytes());
    for pixel in image_data.iter() {
        let r = clamp((pixel.x * max_color as f64) as int, &min_color, &max_color);
        let g = clamp((pixel.y * max_color as f64) as int, &min_color, &max_color);
        let b = clamp((pixel.z * max_color as f64) as int, &min_color, &max_color);

        f.write(format!("{} {} {} ", r, g, b).as_bytes());
    }
}

fn clamp(value: int, min_value: &int, max_value: &int) -> int {
    max(min(value, *max_value), *min_value)
}
