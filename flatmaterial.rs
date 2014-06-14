use vec3::Vec3;
use material::Material;

pub struct FlatMaterial {
    pub color: Vec3
}

impl Material for FlatMaterial {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3) -> Vec3 {
        self.color
    }

    fn is_reflective(&self) -> bool {
        false
    }

    fn global_specular(&self, color: &Vec3) -> Vec3 {
        Vec3::zero()
    }

    fn transmission(&self) -> Vec3 {
        Vec3::zero()
    }
}
