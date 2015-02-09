use std::num::Float;
use vec3::Vec3;
use material::Texture;
use raytracer::compositor::Surface;

#[cfg(test)]
use raytracer::compositor::ColorRGBA;

/// Maps the supplied (u, v) coordinate to the image (s, t).
#[derive(Clone)]
pub struct ImageTexture {
    pub image: Surface
}

impl ImageTexture {
    #[allow(dead_code)]
    pub fn load(filename: &str) -> ImageTexture {
        ImageTexture { image: ::util::import::from_ppm(filename) }
    }

    // Alias, used by skybox sampling. This is needed because we aren't storing the skybox
    // ImageTextures as a more generic Texture (vec of objects with the Texture trait).
    // An ImageTexture-specific function needs to exist to be called.
    pub fn sample(&self, u: f64, v: f64) -> Vec3 {
        self.color(u, v)
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f64, v: f64) -> Vec3 {
        // Avoid out-of-bounds during bilinear filtering
        let s = u % 1.0 * (self.image.width as f64 - 1.0);
        let t = v % 1.0 * (self.image.height as f64 - 1.0);

        let x = s.floor() as usize;
        let y = t.floor() as usize;
        let u_ratio = s - x as f64;
        let v_ratio = t - y as f64;
        let u_opposite = 1.0 - u_ratio;
        let v_opposite = 1.0 - v_ratio;

        (
            (
                  self.image[(x    , y    )].channel_f64() * u_opposite
                + self.image[(x + 1, y    )].channel_f64() * u_ratio
            ) * v_opposite + (
                  self.image[(x    , y + 1)].channel_f64() * u_opposite
                + self.image[(x + 1, y + 1)].channel_f64() * u_ratio
            ) * v_ratio
        ).to_vec3()
    }

    fn clone_self(&self) -> Box<Texture+Send+Sync> {
        let tex: Box<Texture+Send+Sync> = box ImageTexture {
            image: self.image.clone()
        };
        tex
    }
}

#[test]
fn it_bilinearly_filters() {
    let background = ColorRGBA::new_rgb(0, 0, 0);
    let mut surface = Surface::new(2, 2, background);

    surface[(0, 0)] = ColorRGBA::new_rgb(255, 0, 0);
    surface[(0, 1)] = ColorRGBA::new_rgb(0, 255, 0);
    surface[(1, 0)] = ColorRGBA::new_rgb(0, 0, 255);
    surface[(1, 1)] = ColorRGBA::new_rgb(0, 0, 0);

    let texture = ImageTexture { image: surface };

    let left = texture.color(0.0, 0.5);
    assert_eq!(left.x, 0.5);
    assert_eq!(left.y, 0.5);
    assert_eq!(left.z, 0.0);

    let center = texture.color(0.5, 0.5);
    assert_eq!(center.x, 0.25);
    assert_eq!(center.y, 0.25);
    assert_eq!(center.z, 0.25);
}
