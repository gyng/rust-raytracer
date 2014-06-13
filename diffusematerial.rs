use vec3::Vec3;
use material::Material;

pub struct DiffuseMaterial {
    pub k_a: f64,
    pub k_d: f64,
    pub ambient: Vec3,
    pub diffuse: Vec3,

    pub k_s: f64 // testing
}

impl Material for DiffuseMaterial {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3) -> Vec3 {
        let clamped_n_dot_l = n.dot(&l).max(0.0);
        self.ambient.scale(self.k_a) + self.diffuse.scale(clamped_n_dot_l).scale(self.k_d)
    }

    fn is_specular(&self) -> bool {
        self.k_s > 0.0001 // testing
    }

    fn global_specular(&self, color: &Vec3) -> Vec3 {
        color.scale(self.k_s) // testing
    }


    fn transmission(&self) -> Vec3 {
        Vec3 {x: 0.0, y: 0.0, z: 0.0}
    }
}
