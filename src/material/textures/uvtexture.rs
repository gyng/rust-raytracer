use material::Texture;
use raytracer::compositor::{ColorRGBA, Channel};


/// Maps the supplied (u, v) coordinate to the (red, green) color channels.
#[derive(Clone)]
pub struct UVTexture;

impl Texture for UVTexture {
    fn color(&self, u: f64, v: f64) -> ColorRGBA<f64> {
    	let min_value = <f64 as Channel>::min_value();
    	let range = <f64 as Channel>::max_value() - min_value;
    	ColorRGBA::new_rgb(u % range + min_value, v % range + min_value, min_value)
    }

    fn clone_self(&self) -> Box<Texture+Send+Sync> {
        Box::new(UVTexture) as Box<Texture+Send+Sync>
    }
}
