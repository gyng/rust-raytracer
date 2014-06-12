use vec3::Vec3;
use ray::Ray;
use material::Material;

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64
}

impl Material for FlatMaterial {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3) -> Vec3 {
        Vec3 {x: 0, y: 1, z: 0}
    };
}
