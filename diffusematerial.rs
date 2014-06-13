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
        let clamped_n_dot_l = n.dot(&l).max(0.0);
        self.ambient.scale(self.k_a) + self.diffuse.scale(clamped_n_dot_l).scale(self.k_d)
    }

    fn transmission(&self) -> Vec3 {
        Vec3 {x: 0.0, y: 0.0, z: 0.0}
    }
}
