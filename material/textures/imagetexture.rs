use vec3::Vec3;
use material::Texture;
use raytracer::compositor::Surface;


/// Maps the supplied (u, v) coordinate to the image.
#[deriving(Clone)]
pub struct ImageTexture {
    pub image: Surface
}


impl ImageTexture {
    #[allow(dead_code)]
    pub fn load(filename: &str) -> ImageTexture {
        ImageTexture {image: ::util::import::from_ppm(filename)}
    }

    // Alias
    pub fn sample(&self, u: f64, v: f64) -> Vec3 {
        self.color(u, v)
    }
}


impl Texture for ImageTexture {
    // Simple point sampling
    // TODO: Bilinear sampling
    fn color(&self, u: f64, v: f64) -> Vec3 {
        let s = (u % 1.0 * self.image.width as f64) as uint;
        let t = (v % 1.0 * self.image.height as f64) as uint;

        let color = self.image.get(s, t);
        Vec3 {x: color.r as f64 / 255.0, y: color.g as f64 / 255.0, z: color.b as f64 / 255.0}
    }

    fn clone_self(&self) -> Box<Texture+Send+Share> {
        let tex: Box<Texture+Send+Share> = box ImageTexture {
            image: self.image.clone()
        };
        tex
    }
}
