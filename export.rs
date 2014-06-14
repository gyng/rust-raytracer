use std::io::{File, Truncate, Write};
use std::cmp::min;

pub fn to_ppm(image_data: Vec<int>, width: int, height: int, filename: &str) -> () {
    let max_color = 255;
    let header = format!("P3 {} {} {}\n", width, height, max_color);

    // let image_data_string = image_data.iter().map(|&i| i.to_str()).connect(" ");
    // How do I even convert a Vec<int> to a string

    let path = Path::new(filename);
    let mut f = match File::open_mode(&path, Truncate, Write) {
        Ok(f)  => {f},
        Err(e) => fail!("File error: {}", e),
    };

    f.write(header.as_bytes());
    for oct in image_data.iter() {
        f.write_int(*min(oct, &max_color)); // Clamp to ..255
        f.write(" ".as_bytes());
    }
}
