use vec3::Vec3;
use material::Material;

// How do I even use the default trait
pub struct Intersection<'a> {
    pub intersects: bool,
    pub n: Vec3,
    pub t: f64,
    pub position: Vec3,
    pub material: &'a Box<Material>
}
