use geometry::Prim;
use light::Light;
use raytracer::Octree;
use vec3::Vec3;

pub struct Scene {
    pub lights: Vec<Box<Light+Send+Share>>,
    pub prims: Vec<Box<Prim+Send+Share>>,
    pub background: Vec3,
    pub octree: Option<Octree>
}
