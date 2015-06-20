use num::{Float, ToPrimitive};
use std::cmp::{min, max, Ord};
use std::ops::{Add, Mul, Sub};
use vec3::Vec3;

pub trait Channel: ToPrimitive {
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn add(a: Self, b: Self) -> Self;
    fn sub(a: Self, b: Self) -> Self;
}

impl Channel for u8 {
    #[inline]
    fn min_value() -> u8 { u8::min_value() }

    #[inline]
    fn max_value() -> u8 { u8::max_value() }

    #[inline]
    fn add(a: u8, b: u8) -> u8 { a.saturating_add(b) }

    #[inline]
    fn sub(a: u8, b: u8) -> u8 { a.saturating_sub(b) }
}

impl Channel for f64 {
    #[inline]
    fn min_value() -> f64 { 0.0 }

    #[inline]
    fn max_value() -> f64 { 1.0 }

    #[inline]
    fn add(a: f64, b: f64) -> f64 { a + b }

    #[inline]
    fn sub(a: f64, b: f64) -> f64 { a - b }
}


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

// Maybe later?: ColorRGBA<f64>.quantize() -> ColorRGBA<usize>
// How do we implement this more generally so that we may have ColorRGBA<f64>
impl ColorRGBA<u8> {
    pub fn new_rgb_clamped(r: f64, g: f64, b: f64) -> ColorRGBA<u8> {
        let min_color: u8 = Channel::min_value();
        let max_color: u8 = Channel::max_value();

        ColorRGBA::new_rgb(
            clamp((r * max_color as f64).round() as i32, min_color as i32, max_color as i32) as u8,
            clamp((g * max_color as f64).round() as i32, min_color as i32, max_color as i32) as u8,
            clamp((b * max_color as f64).round() as i32, min_color as i32, max_color as i32) as u8)
    }
}

// Maybe later?: ColorRGBA<f64>.quantize() -> ColorRGBA<uint>
// How do we implement this more generally so that we may have ColorRGBA<f64>
impl<T: Channel> ColorRGBA<T> {
    #[allow(dead_code)]
    pub fn new_rgb(r: T, g: T, b: T) -> ColorRGBA<T> {
        ColorRGBA { r: r, g: g, b: b, a: Channel::max_value() }
    }

    #[allow(dead_code)]
    pub fn black() -> ColorRGBA<T> {
        ColorRGBA::new_rgb(
            Channel::min_value(),
            Channel::min_value(),
            Channel::min_value())
    }

    #[allow(dead_code)]
    pub fn white() -> ColorRGBA<T> {
        ColorRGBA::new_rgb(
            Channel::max_value(),
            Channel::max_value(),
            Channel::max_value())
    }

    pub fn channel_f64(&self) -> ColorRGBA<f64> {
        let max_val: T = Channel::max_value();
        ColorRGBA {
            r: self.r.to_f64().unwrap() / max_val.to_f64().unwrap(),
            g: self.g.to_f64().unwrap() / max_val.to_f64().unwrap(),
            b: self.b.to_f64().unwrap() / max_val.to_f64().unwrap(),
            a: self.a.to_f64().unwrap() / max_val.to_f64().unwrap(),
        }
    }

    // Here until we have vec operations (add, mul) for color
    // We also need ColorRGBA<f64>
    pub fn to_vec3(&self) -> Vec3 {
        let color = self.channel_f64();
        Vec3 {
            x: color.r,
            y: color.g,
            z: color.b,
        }
    }
}

impl<T: Channel> Add for ColorRGBA<T> {
    type Output = ColorRGBA<T>;

    fn add(self, other: ColorRGBA<T>) -> ColorRGBA<T> {
        ColorRGBA {
            r: Channel::add(self.r, other.r),
            g: Channel::add(self.g, other.g),
            b: Channel::add(self.b, other.b),
            a: Channel::add(self.a, other.a),
        }
    }
}

impl<T: Channel> Sub for ColorRGBA<T> {
    type Output = ColorRGBA<T>;

    fn sub(self, other: ColorRGBA<T>) -> ColorRGBA<T> {
        ColorRGBA {
            r: Channel::sub(self.r, other.r),
            g: Channel::sub(self.g, other.g),
            b: Channel::sub(self.b, other.b),
            a: Channel::sub(self.a, other.a),
        }
    }
}

impl<T: Float> Mul for ColorRGBA<T> {
    type Output = ColorRGBA<T>;

    fn mul(self, other: ColorRGBA<T>) -> ColorRGBA<T> {
        ColorRGBA {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a
        }
    }
}

// Scalar multiplication
impl<T: Float> Mul<T> for ColorRGBA<T> {
    type Output = ColorRGBA<T>;

    fn mul(self, other: T) -> ColorRGBA<T> {
        ColorRGBA {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
            a: self.a
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

    let foo_color: ColorRGBA<u8> = ColorRGBA::new_rgba(200, 1, 1, 1) +
        ColorRGBA::new_rgba(200, 2, 2, 2);
    assert_eq!(foo_color.r, 255);
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
    let foo_color = ColorRGBA::<f64>::new_rgb(0.5, 0.0, 0.0) * 2.0;

    assert_eq!(foo_color.r, 1.0);
    assert_eq!(foo_color.g, 0.0);
    assert_eq!(foo_color.b, 0.0);
    assert_eq!(foo_color.a, 1.0);
}

