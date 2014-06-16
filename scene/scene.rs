use geometry::Prim;
use light::Light;
use vec3::Vec3;

pub struct Scene {
    pub lights: Vec<Box<Light:Share+Send>>,
    pub prims: Vec<Box<Prim:Share+Send>>,
    pub background: Vec3
}
