use vec3::Vec3;

pub trait Texture {
    fn color(&self, u: f64, v: f64) -> Vec3;
    fn clone_self(&self) -> Box<Texture+Send+Share>;
}

impl Clone for Box<Texture+Send+Share> {
    fn clone(&self) -> Box<Texture+Send+Share> {
        self.clone_self()
    }
}
