use vec3::Vec3;
use material::Material;

// How do I even use the default trait
#[deriving(Default)]
pub struct Intersection<'a> {
    pub intersects: bool,
    pub n: Option<Vec3>,
    pub t: Option<f64>,
    pub position: Option<Vec3>,
    pub material: Option<&'a Box<Material>>
}
