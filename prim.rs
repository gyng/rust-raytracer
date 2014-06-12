use vec3::Vec3;
use ray::Ray;
use material::Material;

pub trait Prim {
    fn intersects(&self, ray: &Ray) -> f64;
    fn material<'a>(&'a self) -> &'a Box<Material>;
}
