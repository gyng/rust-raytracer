use vec3::Vec3;
use material::Material;

pub struct PhongMaterial {
    pub k_a: f64,           // Ambient ratio
    pub k_d: f64,           // Diffuse ratio
    pub k_s: f64,           // Local specular ratio
    pub k_sg: f64,          // Global specular ratio (mirror reflection)
    pub k_tg: f64,          // Global transmissive ratio (refraction)
    pub ambient: Vec3,      // Ambient color
    pub diffuse: Vec3,      // Diffuse color
    pub transmission: Vec3, // Transmissive color
    pub specular: Vec3,     // Specular color
    pub shininess: f64,     // Size of Phong specular highlight
    pub ior: f64            // Index of refraction
}

impl Material for PhongMaterial {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3) -> Vec3 {
        let h = (l + i).unit();

        // Blinn-Phong approximation
        let ambient  = self.ambient.scale(self.k_a);
        let diffuse  = self.diffuse.scale(self.k_d).scale(n.dot(&l));
        let specular = self.specular.scale(self.k_s).scale(n.dot(&h).powf(self.shininess));

        ambient + diffuse + specular
    }

    fn is_reflective(&self) -> bool {
        self.k_sg > 0.0
    }

    fn is_refractive(&self) -> bool {
        self.k_tg > 0.0
    }

    fn global_specular(&self, color: &Vec3) -> Vec3 {
        color.scale(self.k_sg)
    }

    fn global_transmissive(&self, color: &Vec3) -> Vec3 {
        color.scale(self.k_tg)
    }

    fn transmission(&self) -> Vec3 {
        self.transmission
    }

    fn ior(&self) -> f64 {
        self.ior
    }
}
