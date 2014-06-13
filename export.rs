use std::io::{File, Truncate, Write};

pub fn to_ppm(image_data: Vec<int>, width: int, height: int, filename: &str) -> () {
    let max_color = 255;
    let header = format!("P3 {} {} {}\n", width, height, max_color);
    // let image_data_string = image_data.iter().map(|&i| i.to_str()).connect(" ");

    let path = Path::new(filename);
    let mut f = match File::open_mode(&path, Truncate, Write) {
        Ok(f)  => f,
        Err(e) => fail!("File error: {}", e),
    };

    f.write(header.as_bytes());
    // f.write_line(image_data_string);
    for oct in image_data.iter() {
        f.write_int(*oct);
        f.write(" ".as_bytes());
    }
}
