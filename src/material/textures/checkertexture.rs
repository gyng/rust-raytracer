use material::Texture;
use raytracer::compositor::ColorRGBA;


#[derive(Clone)]
pub struct CheckerTexture {
    pub color1: ColorRGBA<f64>,
    pub color2: ColorRGBA<f64>,
    pub scale: f64 // Controls how large the squares are.
}

impl Texture for CheckerTexture {
    fn color(&self, u: f64, v: f64) -> ColorRGBA<f64> {
        let s = (u % self.scale).abs();
        let t = (v % self.scale).abs();
        let half = self.scale / 2.0;

        if s > half && t < half || s < half && t > half {
            self.color1
        } else {
            self.color2
        }
    }

    fn clone_self(&self) -> Box<Texture+Send+Sync> {
        Box::new(CheckerTexture {
            color1: self.color1,
            color2: self.color2,
            scale: self.scale
        }) as Box<Texture+Send+Sync>
    }
}

impl CheckerTexture {
    #[allow(dead_code)]
    pub fn black_and_white(scale: f64) -> CheckerTexture {
        CheckerTexture {
            color1: ColorRGBA::<f64>::black(),
            color2: ColorRGBA::<f64>::white(),
            scale: scale
        }
    }
}
