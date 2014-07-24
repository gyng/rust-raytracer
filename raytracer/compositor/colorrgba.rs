use std::cmp::{min, max, Ord};
use vec3::Vec3;

pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub mod consts {
    pub static MIN_COLOR: u8 = 0_u8;
    pub static MAX_COLOR: u8 = 255_u8;
}

impl Clone for ColorRGBA {
    fn clone(&self) -> ColorRGBA {
        ColorRGBA { r: self.r, g: self.g, b: self.b, a: self.a }
    }
}

fn clamp<T: Ord>(value: T, min_value: T, max_value: T) -> T {
    max(min(value, max_value), min_value)
}


#[allow(dead_code)]
impl ColorRGBA {
    pub fn new_rgb(r: u8, g: u8, b: u8) -> ColorRGBA {
        ColorRGBA { r: r, g: g, b: b, a: consts::MAX_COLOR }
    }

    pub fn new_rgb_clamped(r: f64, g: f64, b: f64) -> ColorRGBA {
        let min_color: int = consts::MIN_COLOR as int;
        let max_color: int = consts::MAX_COLOR as int;

        ColorRGBA::new_rgb(
            clamp((r * max_color as f64).round() as int, min_color, max_color) as u8,
            clamp((g * max_color as f64).round() as int, min_color, max_color) as u8,
            clamp((b * max_color as f64).round() as int, min_color, max_color) as u8)
    }

    pub fn black() -> ColorRGBA {
        ColorRGBA {
            r: consts::MIN_COLOR,
            g: consts::MIN_COLOR,
            b: consts::MIN_COLOR,
            a: consts::MAX_COLOR
        }
    }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> ColorRGBA {
        ColorRGBA { r: r, g: g, b: b, a: a }
    }

    // Here until we have vec operations (add, mul) for color
    pub fn as_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.r as f64 / 255.0,
            y: self.g as f64 / 255.0,
            z: self.b as f64 / 255.0
        }
    }
}
