use std::num::Float;
use std::cmp::{min, max, Ord};
use std::ops::{Add, Mul, Sub};
use vec3::Vec3;

#[derive(Copy)]
pub struct ColorRGBA<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T: Clone> Clone for ColorRGBA<T> {
    fn clone(&self) -> ColorRGBA<T> {
        ColorRGBA {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
            a: self.a.clone()
        }
    }
}

fn clamp<T: Ord>(value: T, min_value: T, max_value: T) -> T {
    max(min(value, max_value), min_value)
}

// Maybe later?: ColorRGBA<f64>.quantize() -> ColorRGBA<uint>
// How do we implement this more generally so that we may have ColorRGBA<f64>
impl ColorRGBA<u8> {
    #[inline]
    pub fn min_value() -> u8 { 0_u8 }

    #[inline]
    pub fn max_value() -> u8 { 255_u8 }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> ColorRGBA<u8> {
        ColorRGBA { r: r, g: g, b: b, a: a }
    }

    #[allow(dead_code)]
    pub fn new_rgb(r: u8, g: u8, b: u8) -> ColorRGBA<u8> {
        ColorRGBA { r: r, g: g, b: b, a: ColorRGBA::max_value() }
    }

    #[allow(dead_code)]
    pub fn black() -> ColorRGBA<u8> {
        ColorRGBA::new_rgba(
            ColorRGBA::min_value(),
            ColorRGBA::min_value(),
            ColorRGBA::min_value(),
            ColorRGBA::min_value())
    }

    pub fn new_rgb_clamped(r: f64, g: f64, b: f64) -> ColorRGBA<u8> {
        let min_color: int = ColorRGBA::min_value() as int;
        let max_color: int = ColorRGBA::max_value() as int;

        ColorRGBA::new_rgb(
            clamp((r * max_color as f64).round() as int, min_color, max_color) as u8,
            clamp((g * max_color as f64).round() as int, min_color, max_color) as u8,
            clamp((b * max_color as f64).round() as int, min_color, max_color) as u8)
    }

    // Here until we have vec operations (add, mul) for color
    // We also need ColorRGBA<f64>
    pub fn as_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.r as f64 / 255.0,
            y: self.g as f64 / 255.0,
            z: self.b as f64 / 255.0
        }
    }
}

impl<T: Add<T, T>> Add<ColorRGBA<T>, ColorRGBA<T>> for ColorRGBA<T> {
    fn add(self, other: ColorRGBA<T>) -> ColorRGBA<T> {
        ColorRGBA {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a
        }
    }
}

impl<T: Sub<T, T>> Sub<ColorRGBA<T>, ColorRGBA<T>> for ColorRGBA<T> {
    fn sub(self, other: ColorRGBA<T>) -> ColorRGBA<T> {
        ColorRGBA {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a
        }
    }
}

impl<T: Mul<T, T>> Mul<ColorRGBA<T>, ColorRGBA<T>> for ColorRGBA<T> {
    fn mul(self, other: ColorRGBA<T>) -> ColorRGBA<T> {
        ColorRGBA {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a
        }
    }
}

#[test]
fn color_add() {
    let foo_color: ColorRGBA<u8> = ColorRGBA::new_rgba(1, 1, 1, 1) +
            ColorRGBA::new_rgba(2, 2, 2, 2);
    assert_eq!(foo_color.r, 3);
    assert_eq!(foo_color.g, 3);
    assert_eq!(foo_color.b, 3);
    assert_eq!(foo_color.a, 3);
}

#[test]
fn color_sub() {
    let foo_color: ColorRGBA<u8> = ColorRGBA::new_rgba(7, 7, 7, 7) -
            ColorRGBA::new_rgba(2, 2, 2, 2);
    assert_eq!(foo_color.r, 5);
    assert_eq!(foo_color.g, 5);
    assert_eq!(foo_color.b, 5);
    assert_eq!(foo_color.a, 5);
}

#[test]
fn color_mul() {
    let foo_color: ColorRGBA<u8> = ColorRGBA::new_rgba(4, 4, 4, 4) *
            ColorRGBA::new_rgba(3, 3, 3, 3);
    assert_eq!(foo_color.r, 12);
    assert_eq!(foo_color.g, 12);
    assert_eq!(foo_color.b, 12);
    assert_eq!(foo_color.a, 12);
}
