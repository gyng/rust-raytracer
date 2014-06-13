use vec3::Vec3;

pub trait Material {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3) -> Vec3;
}

// impl<'a> Clone for &'a Material {
//     fn clone(&self) -> &'a Material {
//         *self
//     }
// }
