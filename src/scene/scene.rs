use light::Light;
use material::textures::CubeMap;
use geometry::Prim;
use raytracer::Octree;
use vec3::Vec3;

pub struct Scene {
    pub lights: Vec<Box<Light+Send+Sync>>,
    pub octree: Octree<Box<Prim+Send+Sync>>,
    pub background: Vec3,
    pub skybox: Option<CubeMap>
}
