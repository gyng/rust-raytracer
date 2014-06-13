use vec3::Vec3;
use material::Material;

pub struct Intersection<'a> {
    pub n: Vec3,
    pub t: f64,
    pub position: Vec3,
    pub material: &'a Box<Material>
}
