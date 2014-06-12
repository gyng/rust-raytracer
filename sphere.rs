use vec3::Vec3;
use ray::Ray;
use material::Material;
use prim::Prim;

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub material: Box<Material>
}

impl Prim for Sphere {
    fn intersects(&self, ray: &Ray) -> f64 {
        0.0
    }

    fn material<'a>(&'a self) -> &'a Box<Material> {
        &self.material
    }
}
