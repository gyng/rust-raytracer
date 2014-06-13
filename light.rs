use vec3::Vec3;

pub trait Light {
    fn position(&self) -> Vec3;
    fn color(&self) -> Vec3;
}

// impl<'a> Clone for &'a Light {
//     fn clone(&self) -> &'a Light {
//         *self
//     }
// }
