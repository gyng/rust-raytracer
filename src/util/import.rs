use geometry::prims::TriangleOptions;
use geometry::{Mesh, Prim};
use image::GenericImage;
use material::materials::CookTorranceMaterial;
use raytracer::compositor::{Surface, ColorRGBA};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use vec3::Vec3;

/// This is limited to only CookTorranceMaterials, as I couldn't get a Box<Material> to clone
/// a new material for each triangle primitive in the object model.
pub fn from_obj(material: CookTorranceMaterial, flip_normals: bool, filename: &str) -> Result<Mesh, String> {
    let file_handle = match File::open(&filename) {
        Ok(f) => f,
        Err(err) => return Err(format!("{}", err))
    };

    let total_bytes = match file_handle.metadata() {
        Ok(metadata) => metadata.len(),
        Err(err) => return Err(format!("{}", err))
    };

    let file = BufReader::new(file_handle);

    let start_time = ::time::get_time();
    let print_every = 2048u32;
    let mut current_line = 0;
    let mut processed_bytes = 0;

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut normals : Vec<Vec3> = Vec::new();
    let mut triangles: Vec<Box<Prim+Send+Sync>> = Vec::new();
    let mut tex_coords: Vec<Vec<f64>> = Vec::new();
    let normal_scale = if flip_normals { -1.0 } else { 1.0 };

    for line_iter in file.lines() {
        let line = line_iter.unwrap();
        let tokens: Vec<&str> = line[..].split_whitespace().collect();
        if tokens.len() == 0 { continue }

        match tokens[0] {
            "v" => {
                vertices.push(Vec3 {
                    x: tokens[1].parse().unwrap(),
                    y: tokens[2].parse().unwrap(),
                    z: tokens[3].parse().unwrap()
                });
            },
            "vt" => {
                tex_coords.push(vec![
                    tokens[1].parse().unwrap(),
                    tokens[2].parse().unwrap()
                ]);
            },
            "vn" => {
                normals.push(Vec3 {
                    x: tokens[1].parse::<f64>().unwrap() * normal_scale,
                    y: tokens[2].parse::<f64>().unwrap() * normal_scale,
                    z: tokens[3].parse::<f64>().unwrap() * normal_scale
                });
            },
            "f" => {
                // ["f", "1/2/3", "2/2/2", "12//4"] => [[1, 2, 3], [2, 2, 2], [12, -1u, 4]]
                let tail = match tokens.split_first() {
                    Some((_, tail)) => tail,
                    None => return Err("Face syntax of OBJ not supported or malformed".to_owned())
                };

                let pairs: Vec<Vec<usize>> = tail.iter().map( |token| {
                    let str_tokens: Vec<&str> = token.split('/').collect();
                    str_tokens.iter().map( |str_tok| {
                        match str_tok.parse::<usize>().ok() {
                            Some(usize_tok) => usize_tok - 1, // Have to offset as OBJ is 1-indexed
                            None => !0 // No data available/not supplied (eg. `//` as a token)
                        }
                    }).collect()
                }).collect();

                // If no texture coordinates were supplied, default to zero.
                // We stored nothing supplied as !0
                let (u, v) = if pairs[0][1] != !0 {
                    (vec![
                        tex_coords[pairs[0][1]][0],
                        tex_coords[pairs[1][1]][0],
                        tex_coords[pairs[2][1]][0]
                    ],
                    vec![
                        tex_coords[pairs[0][1]][1],
                        tex_coords[pairs[1][1]][1],
                        tex_coords[pairs[2][1]][1]
                    ])
                } else {
                    (vec![0.0, 0.0, 0.0],
                     vec![0.0, 0.0, 0.0])
                };

                let mut triopts = TriangleOptions::new(
                    vertices[pairs[0][0]],
                    vertices[pairs[1][0]],
                    vertices[pairs[2][0]]);

                triopts.material(Box::new(material.clone()));
                triopts.normals([
                    normals[pairs[0][2]],
                    normals[pairs[1][2]],
                    normals[pairs[2][2]],
                ]);
                triopts.texinfo([(u[0], v[0]), (u[1], v[1]), (u[2], v[2])]);

                triangles.push(Box::new(triopts.build()));
            },
            _ => {}
        }

        current_line += 1;
        processed_bytes += line.as_bytes().len();
        if current_line % print_every == 0 {
            ::util::print_progress("Bytes", start_time.clone(), processed_bytes, total_bytes as usize);
        }
    }

    // Cheat the progress meter
    ::util::print_progress("Bytes", start_time, total_bytes as usize, total_bytes as usize);

    Ok(Mesh { triangles: triangles })
}

pub fn from_image<P: AsRef<Path>>(path: P) -> Result<Surface, String> {
    let image = match ::image::open(path) {
        Ok(image) => image.to_rgba(),
        Err(err) => return Err(format!("{}", err))
    };

    let mut surface = Surface::new(image.width() as usize,
                                   image.height() as usize,
                                   ColorRGBA::transparent());

    for (src, dst_pixel) in image.pixels().zip(surface.iter_pixels_mut()) {
        *dst_pixel = ColorRGBA::new_rgba(src[0], src[1], src[2], src[3]);
    }

    Ok(surface)
}

#[test]
pub fn test_obj_loads_correct_number_of_triangles() {
    let material: CookTorranceMaterial = Default::default();
    let mesh = from_obj(material, false, "test/res/cube.obj")
            .ok().expect("failed to laod test obj `test/res/cube.obj`");

    assert_eq!(mesh.triangles.len(), 12);
}

#[test]
pub fn test_from_png24() {
    let surface = from_image("test/res/png24.png")
            .ok().expect("failed to load test image `test/res/png24.png`");

    let expected_image: [[(u8, u8, u8, u8); 10]; 2] = [[
        (0, 0, 0, 255), (1, 1, 1, 255), (2, 2, 2, 255),
        (3, 3, 3, 255), (4, 4, 4, 255), (5, 5, 5, 255),
        (6, 6, 6, 255), (7, 7, 7, 255), (8, 8, 8, 255),
        (9, 9, 9, 255)
    ], [
        (255, 0, 0, 255), (255, 0, 0, 127), (255, 0, 0, 0),
        (0, 255, 0, 255), (0, 255, 0, 127), (0, 255, 0, 0),
        (0, 0, 255, 255), (0, 0, 255, 127), (0, 0, 255, 0),
        (0, 0, 0, 0)
    ]];

    for y in (0..1) {
        for x in (0..9) {
            let pixel = surface[(x, y)];
            let expected = expected_image[y][x];
            assert_eq!(expected, (pixel.r, pixel.g, pixel.b, pixel.a));
        }
    }
}
