use raytracer::compositor::ColorRGBA;

pub trait Texture {
    fn color(&self, u: f64, v: f64) -> ColorRGBA<f64>;
    fn clone_self(&self) -> Box<Texture+Send+Sync>;
}

impl Clone for Box<Texture+Send+Sync> {
    fn clone(&self) -> Box<Texture+Send+Sync> {
        self.clone_self()
    }
}
