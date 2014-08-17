use light::Light;
use material::textures::CubeMap;
use raytracer::PrimContainer;
use vec3::Vec3;

pub struct Scene {
    pub lights: Vec<Box<Light+Send+Sync>>,
    pub prim_strat: Box<PrimContainer+Send+Sync>,
    pub background: Vec3,
    pub skybox: Option<CubeMap>
}
