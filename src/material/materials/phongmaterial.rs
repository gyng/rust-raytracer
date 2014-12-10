use std::num::Float;
use material::{Material, Texture};
use vec3::Vec3;

#[allow(dead_code)]
#[deriving(Clone)]
pub struct PhongMaterial {
    pub k_a: f64,           // Ambient coefficient
    pub k_d: f64,           // Diffuse coefficient
    pub k_s: f64,           // Local specular coefficient
    pub k_sg: f64,          // Global specular coefficient (mirror reflection)
    pub k_tg: f64,          // Global transmissive coefficient (refraction)
    pub ambient: Vec3,      // Ambient color
    pub diffuse: Vec3,      // Diffuse color
    pub transmission: Vec3, // Transmissive color
    pub specular: Vec3,     // Specular color
    pub shininess: f64,     // Size of Phong specular highlight
    pub ior: f64,           // Index of refraction
    pub diffuse_texture: Option<Box<Texture+Send+Sync>>
}

impl Material for PhongMaterial {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3, u: f64, v: f64) -> Vec3 {
        // let h = (l + i).unit();

        // Blinn-Phong approximation
        let ambient  = self.ambient.scale(self.k_a);
        let diffuse  = self.diffuse.scale(self.k_d).scale(n.dot(&l)) * match self.diffuse_texture {
            Some(ref x) => x.color(u, v),
            None => Vec3::one()
        };
        let specular = self.specular.scale(self.k_s).scale(self.brdf(&n, &i, &l));

        ambient + diffuse + specular
    }

    fn brdf(&self, n: &Vec3, i: &Vec3, l: &Vec3) -> f64 {
        let h = (l + *i).unit();
        n.dot(&h).powf(self.shininess)
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

    // TODO: Move sample code into brdf
    fn brdf(&self, n: Vec3, incoming: Vec3, outgoing: Vec3, u: f64, v: f64) -> Vec3 {
        self.sample(n, outgoing, incoming, u, v) - self.ambient.scale(self.k_a)
    }
}
