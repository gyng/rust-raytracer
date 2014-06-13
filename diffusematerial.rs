use vec3::Vec3;
use material::Material;

pub struct DiffuseMaterial {
    pub k_a: f64,
    pub k_d: f64,
    pub ambient: Vec3,
    pub diffuse: Vec3
}

impl Material for DiffuseMaterial {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3) -> Vec3 {
      self.ambient.scale(self.k_a) + self.diffuse.scale(n.dot(&l)).scale(self.k_d)
    }
}
