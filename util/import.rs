use std::io::{BufferedReader, File};
use material::Material;
use geometry::{Mesh, Prim};
use geometry::prims::Triangle;
use vec3::Vec3;

#[allow(dead_code)]
pub fn from_obj(position: Vec3, scale: f64, material: Box<Material:Send+Share>, filename: &str) -> Mesh {
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

        let tokens: Vec<&str> = line.as_slice().split(' ').collect();
        if tokens.len() < 1 { continue }

        match tokens.get(0).as_slice() {
            "v" => {
                vertices.push(Vec3 {
                    x: match from_str::<f64>(tokens.get(1).as_slice()) { Some(f) => f, None => fail!("Bad vertex coordinate in file.") },
                    y: match from_str::<f64>(tokens.get(2).as_slice()) { Some(f) => f, None => fail!("Bad vertex coordinate in file.") },
                    z: match from_str::<f64>(tokens.get(3).as_slice()) { Some(f) => f, None => fail!("Bad vertex coordinate in file.") },
                });
            },
            "vn" => {
                normals.push(Vec3 {
                    x: match from_str::<f64>(tokens.get(1).as_slice()) { Some(f) => f, None => fail!("Bad vertex coordinate in file.") },
                    y: match from_str::<f64>(tokens.get(2).as_slice()) { Some(f) => f, None => fail!("Bad vertex coordinate in file.") },
                    z: match from_str::<f64>(tokens.get(3).as_slice()) { Some(f) => f, None => fail!("Bad vertex coordinate in file.") },
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
                    material: material.clone()
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
