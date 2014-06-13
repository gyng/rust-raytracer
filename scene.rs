use vec3::Vec3;
use prim::Prim;
use light::Light;

// #[deriving(Clone)]
pub struct Scene {
    pub lights: Vec<Box<Light>>,
    pub prims: Vec<Box<Prim>>,
    pub background: Vec3
}

// impl Clone for Box<Light> {
//     fn clone(&self) -> Box<Light> {
//         *self
//     }
// }

// impl Clone for Box<Prim> {
//     fn clone(&self) -> Box<Prim> {
//         *self
//     }
// }
