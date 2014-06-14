use vec3::Vec3;
use material::Material;

pub struct PhongMaterial {
    pub k_a: f64,       // Ambient ratio
    pub k_d: f64,       // Diffuse ratio
    pub k_s: f64,       // Local specular ratio
    pub k_sg: f64,      // Global specular ratio (mirror)
    pub ambient: Vec3,  // Ambient color
    pub diffuse: Vec3,  // Diffuse color
    pub specular: Vec3, // Specular color
    pub shininess: f64  // Size of Phong specular highlight
}

impl Material for PhongMaterial {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3) -> Vec3 {
        let n_dot_l = n.dot(&l);
        let h = (l + i).unit();

        // Blinn-Phong approximation
        let ambient  = self.ambient.scale(self.k_a);
        let diffuse  = self.diffuse.scale(self.k_d).scale(n_dot_l);
        let specular = self.specular.scale(self.k_s).scale(n.dot(&h).powf(self.shininess));

        ambient + diffuse + specular
    }

    fn is_reflective(&self) -> bool {
        self.k_sg > ::std::f64::EPSILON
    }

    fn global_specular(&self, color: &Vec3) -> Vec3 {
        color.scale(self.k_sg) // testing
    }

    fn transmission(&self) -> Vec3 {
        Vec3 {x: 0.0, y: 0.0, z: 0.0}
    }
}
