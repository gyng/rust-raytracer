use std::io::{BufferedReader, File};
use material::materials::CookTorranceMaterial;
use geometry::{Mesh, Prim};
use geometry::prims::Triangle;
use vec3::Vec3;

/// This is limited to only CookTorranceMaterials, as I couldn't get a Box<Material> to clone
/// a new material for each triangle primitive in the object model.
#[allow(dead_code)]
pub fn from_obj(position: Vec3, scale: f64, material: CookTorranceMaterial /*Box<Material>*/, filename: &str) -> Mesh {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut triangles: Vec<Box<Prim:Send+Share>> = Vec::new();

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
                    x: match from_str::<f64>(tokens.get(1).as_slice()) { Some(f) => f, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) },
                    y: match from_str::<f64>(tokens.get(2).as_slice()) { Some(f) => f, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) },
                    z: match from_str::<f64>(tokens.get(3).as_slice()) { Some(f) => f, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) },
                });
            },
            "vn" => {
                normals.push(Vec3 {
                    x: match from_str::<f64>(tokens.get(1).as_slice()) { Some(f) => f, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) },
                    y: match from_str::<f64>(tokens.get(2).as_slice()) { Some(f) => f, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) },
                    z: match from_str::<f64>(tokens.get(3).as_slice()) { Some(f) => f, None => fail!(format!("Bad vertex coordinate in file. `{}`", line)) },
                });
            },
            "f" => {
                // let pairs: Vec<&str> = tokens.tail().iter().map( |token| {
                //     token.to_string().as_slice().split('/').collect()
                // }).collect();

                let v0: Vec<&str> = tokens.get(1).split('/').collect();
                let v1: Vec<&str> = tokens.get(2).split('/').collect();
                let v2: Vec<&str> = tokens.get(3).split('/').collect();

                 triangles.push(box Triangle {
                    v0: match from_str::<int>(*v0.get(0)) { Some(x) => {*vertices.get((x - 1) as uint)}, None => {fail!("Bad vertex")} },
                    v1: match from_str::<int>(*v1.get(0)) { Some(x) => {*vertices.get((x - 1) as uint)}, None => {fail!("Bad vertex")} },
                    v2: match from_str::<int>(*v2.get(0)) { Some(x) => {*vertices.get((x - 1) as uint)}, None => {fail!("Bad vertex")} },
                    n0: match from_str::<int>(*v0.get(2)) { Some(x) => {*normals.get((x - 1) as uint)}, None => {fail!("Bad normal")} },
                    n1: match from_str::<int>(*v1.get(2)) { Some(x) => {*normals.get((x - 1) as uint)}, None => {fail!("Bad normal")} },
                    n2: match from_str::<int>(*v2.get(2)) { Some(x) => {*normals.get((x - 1) as uint)}, None => {fail!("Bad normal")} },
                    material: box material.clone() /*box material.clone()*/
                });
            },
            _ => {}
        }
    }

    Mesh {
        position: position,
        scale: scale,
        triangles: triangles
    }
}
