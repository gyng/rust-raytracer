use std::io::{BufferedReader, File};
use material::materials::CookTorranceMaterial;
use geometry::{Mesh, Prim};
use geometry::prims::Triangle;
use raytracer::compositor::{Surface, ColorRGBA};
use vec3::Vec3;

/// This is limited to only CookTorranceMaterials, as I couldn't get a Box<Material> to clone
/// a new material for each triangle primitive in the object model.
#[allow(dead_code)]
pub fn from_obj(position: Vec3, scale: f64, material: CookTorranceMaterial /*Box<Material>*/, flip_normals: bool, filename: &str) -> Mesh {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));

    let mut vertices:   Vec<Vec3> = Vec::new();
    let mut tex_coords: Vec<Vec3> = Vec::new();
    let mut normals:    Vec<Vec3> = Vec::new();
    let mut triangles:  Vec<Box<Prim+Send+Share>> = Vec::new();

    let start_time = ::time::get_time();
    let print_progress_every = 1024u;
    let mut current_line = 0;
    let mut processed_bytes = 0;
    let total_bytes = match path.stat() {
        Ok(stat) => stat.size,
        Err(e) => fail!(e)
    };

    for line_iter in file.lines() {
        let line: String = match line_iter {
            Ok(x) => x,
            Err(e) => fail!(e)
        };

        let tokens: Vec<&str> = line.as_slice().words().collect();
        if tokens.len() < 4 { continue }

        match tokens[0].as_slice() {
            "v" => {
                vertices.push(Vec3 {
                    x: parse_coord_str(tokens[1].as_slice(), scale, line.as_slice()),
                    y: parse_coord_str(tokens[2].as_slice(), scale, line.as_slice()),
                    z: parse_coord_str(tokens[3].as_slice(), scale, line.as_slice()),
                });
            },
            "vt" => {
                tex_coords.push(Vec3 {
                    x: parse_coord_str(tokens[1].as_slice(), scale, line.as_slice()),
                    y: parse_coord_str(tokens[2].as_slice(), scale, line.as_slice()),
                    z: 0.0
                });
            },
            "vn" => {
                let normals_flip_scale = if flip_normals { -1.0 } else { 1.0 } * scale;
                normals.push(Vec3 {
                    x: parse_coord_str(tokens[1].as_slice(), scale * normals_flip_scale, line.as_slice()),
                    y: parse_coord_str(tokens[2].as_slice(), scale * normals_flip_scale, line.as_slice()),
                    z: parse_coord_str(tokens[3].as_slice(), scale * normals_flip_scale, line.as_slice()),
                });
            },
            "f" => {
                // ["f", "1/2/3", "2/2/2", "12//4"] => [[1, 2, 3], [2, 2, 2], [12, -1u, 4]]
                let pairs: Vec<Vec<uint>> = tokens.tail().iter().map( |token| {
                    let str_tokens: Vec<&str> = token.as_slice().split('/').collect();

                    str_tokens.iter().map( |str_tok| {
                        match from_str::<uint>(*str_tok) {
                            Some(uint_tok) => uint_tok - 1,
                            None => -1 // No data available/not supplied
                        }
                    }).collect()
                }).collect();

                // let v0: Vec<&str> = tokens[1].split('/').collect();
                // let v1: Vec<&str> = tokens[2].split('/').collect();
                // let v2: Vec<&str> = tokens[3].split('/').collect();

                // If no texture coordinates were supplied, default to zero.
                let mut u = Vec3::zero();
                let mut v = Vec3::zero();

                // We store nothing supplied as -1 (uint=4294967295)
                if pairs[0][1] != 4294967295 {
                    u = Vec3 {
                        x: tex_coords[pairs[0][1]].x,
                        y: tex_coords[pairs[1][1]].x,
                        z: tex_coords[pairs[2][1]].x
                    };

                    v = Vec3 {
                        x: tex_coords[pairs[0][1]].y,
                        y: tex_coords[pairs[1][1]].y,
                        z: tex_coords[pairs[2][1]].y
                    };
                }

                triangles.push(box Triangle {
                    v0: vertices[pairs[0][0]],
                    v1: vertices[pairs[1][0]],
                    v2: vertices[pairs[2][0]],

                    n0: normals[pairs[0][2]],
                    n1: normals[pairs[1][2]],
                    n2: normals[pairs[2][2]],

                    u: u,
                    v: v,
                    material: box material.clone()
                });
            },
            _ => {}
        }

        current_line += 1;
        processed_bytes += line.as_bytes().len();
        if current_line % print_progress_every == 0 {
            ::util::print_progress("Bytes", start_time, processed_bytes, total_bytes as uint);
        }
    }

    // Cheat the progress meter
    ::util::print_progress("Bytes", start_time, total_bytes as uint, total_bytes as uint);

    Mesh {
        position: position,
        scale: scale,
        triangles: triangles
    }
}

fn parse_coord_str(coord: &str, scale: f64, line: &str) -> f64 {
    match from_str::<f64>(coord.as_slice()) {
        Some(f) => f * scale,
        None => fail!(format!("Bad vertex or texture coordinate in file. `{}`", line))
    }
}


#[allow(dead_code)]
pub fn from_ppm(filename: &str) -> Surface {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));

    let tex = match file.read_to_string() { Ok(f) => f, Err(e) => { println!("Could not open {}", filename); fail!(e) }};
    let mut tokens: Vec<&str> = tex.as_slice().words().collect();

    tokens.remove(0); // PPM type
    let width  = match tokens.remove(0) { Some(x) => uint_from_string(x), None => fail!("Bad PPM") };
    let height = match tokens.remove(0) { Some(x) => uint_from_string(x), None => fail!("Bad PPM") };
    tokens.remove(0); // Max color value

    print!("Importing image texture {}", filename);
    println!(" {}x{}", width, height);

    let mut surface = Surface::new(width, height, ColorRGBA::new_rgb(0, 0, 0));

    let mut i = 0u;

    for chunk in tokens.as_slice().chunks(3) {
        let x = i % width;
        let y = i / width;
        i += 1;

        if x >= width || y >= height { break };

        *surface.get_mut(x, y) = ColorRGBA::new_rgb(
            uint_from_string(chunk[0]) as u8,
            uint_from_string(chunk[1]) as u8,
            uint_from_string(chunk[2]) as u8
        );
    }

    surface
}

fn uint_from_string(s: &str) -> uint {
     match from_str::<uint>(s) {
        Some(x) => x,
        None => fail!("Bad uint string {}", s)
    }
}
