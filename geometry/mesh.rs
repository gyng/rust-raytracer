use geometry::Prim;
use vec3::Vec3;

#[allow(dead_code)]
pub struct Mesh {
    pub position: Vec3, // Unimplemented
    // rotation: Quaternion,
    pub scale: f64, // Unimplemented
    pub triangles: Vec<Box<Prim:Send+Share>>
}
