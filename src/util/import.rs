use std::io::{BufferedReader, File};
use material::materials::CookTorranceMaterial;
use geometry::{Mesh, Prim};
use geometry::prims::{Triangle, TriangleVertex};
use raytracer::compositor::{Surface, ColorRGBA};
use vec3::Vec3;

/// This is limited to only CookTorranceMaterials, as I couldn't get a Box<Material> to clone
/// a new material for each triangle primitive in the object model.
#[allow(dead_code)]
pub fn from_obj(position: Vec3, scale: f64, material: CookTorranceMaterial /*Box<Material>*/, flip_normals: bool, filename: &str) -> Mesh {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut tex_coords: Vec<Vec<f64>> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut triangles: Vec<Box<Prim+Send+Share>> = Vec::new();

    let start_time = ::time::get_time();
    let print_progress_every = 1024u;
    let mut current_line = 0;
    let mut processed_bytes = 0;
    let total_bytes = match path.stat() {
        Ok(stat) => stat.size,
        Err(e) => fail!("Could not open file {} (file missing?): {}", filename, e)
    };

    for line_iter in file.lines() {
        let line: String = match line_iter {
            Ok(x) => x,
            Err(e) => fail!("Could not open file {} (file missing?): {}", filename, e)
        };

        let tokens: Vec<&str> = line.as_slice().words().collect();
        if tokens.len() < 4 { continue }

        match tokens[0].as_slice() {
            "v" => {
                vertices.push(Vec3{
                    x: parse_coord_str(tokens[1].as_slice(), scale, line.as_slice()),
                    y: parse_coord_str(tokens[2].as_slice(), scale, line.as_slice()),
                    z: parse_coord_str(tokens[3].as_slice(), scale, line.as_slice()),
                });
            },
            "vt" => {
                tex_coords.push(vec![
                    parse_coord_str(tokens[1].as_slice(), scale, line.as_slice()),
                    parse_coord_str(tokens[2].as_slice(), scale, line.as_slice())
                ]);
            },
            "vn" => {
                let normals_flip_scale = if flip_normals { -1.0 } else { 1.0 } * scale;
                normals.push(Vec3{
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
                            None => !0 // No data available/not supplied
                        }
                    }).collect()
                }).collect();

                // If no texture coordinates were supplied, default to zero.
                let mut u = vec![0.0, 0.0, 0.0];
                let mut v = vec![0.0, 0.0, 0.0];

                // We store nothing supplied as !0
                if pairs[0][1] != !0 {
                    u = vec![
                        tex_coords[pairs[0][1]][0],
                        tex_coords[pairs[1][1]][0],
                        tex_coords[pairs[2][1]][0]
                    ];

                    v = vec![
                        tex_coords[pairs[0][1]][1],
                        tex_coords[pairs[1][1]][1],
                        tex_coords[pairs[2][1]][1]
                    ];
                }

                triangles.push(box Triangle {
                    v0: TriangleVertex { pos: vertices[pairs[0][0]], n: normals[pairs[0][2]], u: u[0], v: v[0] },
                    v1: TriangleVertex { pos: vertices[pairs[1][0]], n: normals[pairs[1][2]], u: u[1], v: v[1] },
                    v2: TriangleVertex { pos: vertices[pairs[2][0]], n: normals[pairs[2][2]], u: u[2], v: v[2] },
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

    let tex = match file.read_to_string() {
        Ok(f) => f,
        Err(e) => fail!("Could not open file {} (file missing?): {}", filename, e)
    };
    let mut tokens: Vec<&str> = tex.as_slice().words().collect();

    tokens.remove(0); // PPM type
    let width  = from_str::<uint>(tokens.remove(0).unwrap()).unwrap();
    let height = from_str::<uint>(tokens.remove(0).unwrap()).unwrap();
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
            from_str::<u8>(chunk[0]).unwrap(),
            from_str::<u8>(chunk[1]).unwrap(),
            from_str::<u8>(chunk[2]).unwrap()
        );
    }

    surface
}
