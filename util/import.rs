use std::io::{BufferedReader, File};
use material::materials::CookTorranceMaterial;
use geometry::{Mesh, Prim};
use geometry::prims::Triangle;
use vec3::Vec3;

/// This is limited to only CookTorranceMaterials, as I couldn't get a Box<Material> to clone
/// a new material for each triangle primitive in the object model.
#[allow(dead_code)]
pub fn from_obj(position: Vec3, scale: f64, material: CookTorranceMaterial /*Box<Material>*/, flip_normals: bool, filename: &str) -> Mesh {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut tex_coords: Vec<Vec3> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut triangles: Vec<Box<Prim+Send+Share>> = Vec::new();

    let start_time = ::time::get_time();
    let print_progress_every = 1024u;
    let mut current_line = 0;
    let mut processed_bytes = 0;
    let total_bytes = match path.stat() {
        Ok(stat) => stat.size,
        Err(e) => fail!(e),
    };

    for line_iter in file.lines() {
        let line: String = match line_iter {
            Ok(x) => {x},
            Err(e) => {fail!(e)}
        };

        let tokens: Vec<&str> = line.as_slice().words().collect();
        if tokens.len() < 4 { continue }

        match tokens.get(0).as_slice() {
            "v" => {
                vertices.push(Vec3 {
                    x: match from_str::<f64>(tokens.get(1).as_slice()) { Some(f) => f * scale, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) },
                    y: match from_str::<f64>(tokens.get(2).as_slice()) { Some(f) => f * scale, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) },
                    z: match from_str::<f64>(tokens.get(3).as_slice()) { Some(f) => f * scale, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) }
                });
            },
            "vt" => {
                tex_coords.push(Vec3 {
                    x: match from_str::<f64>(tokens.get(1).as_slice()) { Some(f) => f * scale, None => fail!(format!("Bad texture coordinate in file. w-coord not supported. `{}`", line)) },
                    y: match from_str::<f64>(tokens.get(2).as_slice()) { Some(f) => f * scale, None => fail!(format!("Bad texture coordinate in file. w-coord not supported. `{}`", line)) },
                    z: 0.0
                });
            },
            "vn" => {
                let normals_flip_scale = if flip_normals { -1.0 } else { 1.0 };
                normals.push(Vec3 {
                    x: match from_str::<f64>(tokens.get(1).as_slice()) { Some(f) => f * scale * normals_flip_scale, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) },
                    y: match from_str::<f64>(tokens.get(2).as_slice()) { Some(f) => f * scale * normals_flip_scale, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) },
                    z: match from_str::<f64>(tokens.get(3).as_slice()) { Some(f) => f * scale * normals_flip_scale, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) }
                });
            },
            "f" => {
                // let pairs: Vec<&str> = tokens.tail().iter().map( |token| {
                //     token.to_string().as_slice().split('/').collect()
                // }).collect();

                let v0: Vec<&str> = tokens.get(1).split('/').collect();
                let v1: Vec<&str> = tokens.get(2).split('/').collect();
                let v2: Vec<&str> = tokens.get(3).split('/').collect();

                // If no texture coordinates were supplied, default to zero.
                let mut u = Vec3::zero();
                let mut v = Vec3::zero();

                if v0.get(1).len() > 0 {
                    u = Vec3 {
                        x: match from_str::<int>(*v0.get(1)) { Some(x) => {tex_coords.get((x - 1) as uint).x}, None => {fail!("Bad texture")} },
                        y: match from_str::<int>(*v1.get(1)) { Some(x) => {tex_coords.get((x - 1) as uint).x}, None => {fail!("Bad texture")} },
                        z: match from_str::<int>(*v2.get(1)) { Some(x) => {tex_coords.get((x - 1) as uint).x}, None => {fail!("Bad texture")} }
                    };

                    v = Vec3 {
                        x: match from_str::<int>(*v0.get(1)) { Some(x) => {tex_coords.get((x - 1) as uint).y}, None => {fail!("Bad texture")} },
                        y: match from_str::<int>(*v1.get(1)) { Some(x) => {tex_coords.get((x - 1) as uint).y}, None => {fail!("Bad texture")} },
                        z: match from_str::<int>(*v2.get(1)) { Some(x) => {tex_coords.get((x - 1) as uint).y}, None => {fail!("Bad texture")} }
                    };
                }

                triangles.push(box Triangle {
                    v0: match from_str::<int>(*v0.get(0)) { Some(x) => {*vertices.get((x - 1) as uint)}, None => {fail!("Bad vertex")} },
                    v1: match from_str::<int>(*v1.get(0)) { Some(x) => {*vertices.get((x - 1) as uint)}, None => {fail!("Bad vertex")} },
                    v2: match from_str::<int>(*v2.get(0)) { Some(x) => {*vertices.get((x - 1) as uint)}, None => {fail!("Bad vertex")} },

                    n0: match from_str::<int>(*v0.get(2)) { Some(x) => {*normals.get((x - 1) as uint)}, None => {fail!("Bad normal")} },
                    n1: match from_str::<int>(*v1.get(2)) { Some(x) => {*normals.get((x - 1) as uint)}, None => {fail!("Bad normal")} },
                    n2: match from_str::<int>(*v2.get(2)) { Some(x) => {*normals.get((x - 1) as uint)}, None => {fail!("Bad normal")} },

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
