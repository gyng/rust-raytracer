use material::Material;
use vec3::Vec3;

#[allow(dead_code)]
#[derive(Clone)]
pub struct FlatMaterial {
    pub color: Vec3
}

impl Material for FlatMaterial {
    fn sample(&self, _n: Vec3, _i: Vec3, _l: Vec3, _u: f64, _v: f64) -> Vec3 {
        self.color
    }

    fn is_reflective(&self) -> bool {
        false
    }

    fn is_refractive(&self) -> bool {
        false
    }

    fn global_specular(&self, _color: &Vec3) -> Vec3 {
        Vec3::zero()
    }

    fn global_transmissive(&self, _color: &Vec3) -> Vec3 {
        Vec3::zero()
    }

    fn transmission(&self) -> Vec3 {
        Vec3::zero()
    }

    fn ior(&self) -> f64 {
        1.0
    }

    fn is_glossy(&self) -> bool {
        false
    }

    fn glossiness(&self) -> f64 {
        0.0
    }
}

impl Default for FlatMaterial {
    fn default() -> FlatMaterial {
        FlatMaterial { color: Vec3 { x: 0.5, y: 0.5, z: 0.5 } }
    }
}
