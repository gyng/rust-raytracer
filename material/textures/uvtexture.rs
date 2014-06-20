use vec3::Vec3;
use material::Texture;


/// Maps the supplied (u, v) coordinate to the (red, green) color channels.
#[deriving(Clone)]
pub struct UVTexture;


impl Texture for UVTexture {
    fn color(&self, u: f64, v: f64) -> Vec3 {
        Vec3 {x: u % 1.0, y: v % 1.0, z: 0.0}
    }

    fn clone_self(&self) -> Box<Texture+Send+Share> {
        let tex: Box<Texture+Send+Share> = box UVTexture;
        tex
    }
}

impl Clone for Box<Texture+Send+Share> {
    fn clone(&self) -> Box<Texture+Send+Share> {
        self.clone_self()
    }
}
