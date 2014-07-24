use vec3::Vec3;
use material::Texture;
use raytracer::compositor::Surface;


/// Maps the supplied (u, v) coordinate to the image (s, t).
#[deriving(Clone)]
pub struct ImageTexture {
    pub image: Surface
}


impl ImageTexture {
    #[allow(dead_code)]
    pub fn load(filename: &str) -> ImageTexture {
        ImageTexture {image: ::util::import::from_ppm(filename)}
    }

    // Alias, used by skybox sampling. This is needed because we aren't storing the skybox ImageTextures
    // as a more generic Texture (vec of objects with the Texture trait). We need an ImageTexture-specific
    // function to call.
    pub fn sample(&self, u: f64, v: f64) -> Vec3 {
        self.color(u, v)
    }
}


impl Texture for ImageTexture {
    fn color(&self, u: f64, v: f64) -> Vec3 {
        // Don't want any out-of-bounds during bilinear filtering
        let s = u % 1.0 * (self.image.width as f64 - 1.0);
        let t = v % 1.0 * (self.image.height as f64 - 1.0);

        // Get nearest neighbours for bilinear filtering (avoiding edges)
        let x = s.floor() as uint;
        let y = t.floor() as uint;
        let u_ratio = s - x as f64;
        let v_ratio = t - y as f64;
        let u_opposite = 1.0 - u_ratio;
        let v_opposite = 1.0 - v_ratio;

        (self.image.get(x, y    ).as_vec3().scale(u_opposite) + self.image.get(x + 1, y    ).as_vec3().scale(u_ratio)).scale(v_opposite) +
        (self.image.get(x, y + 1).as_vec3().scale(u_opposite) + self.image.get(x + 1, y + 1).as_vec3().scale(u_ratio)).scale(v_ratio)
    }

    fn clone_self(&self) -> Box<Texture+Send+Share> {
        let tex: Box<Texture+Send+Share> = box ImageTexture {
            image: self.image.clone()
        };
        tex
    }
}
