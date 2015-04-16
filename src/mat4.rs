#![allow(dead_code)]

use geometry::bbox::BBox;
use raytracer::Ray;
use std::cmp;
use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, Mul, Sub};
use vec3::Vec3;

/// Stored in row-major, M_(i, j) = i-th row and j-th column
/// 0-indexed
#[derive(Clone, Copy)]
pub struct Mat4 {
    pub m: [[f64; 4]; 4]
}

/// We store the inverse matrix for convenience as per pbrt's recommendation
pub struct Transform {
    pub m: Mat4,
    pub inv: Mat4
}

impl Transform {
    pub fn new(mat: Mat4) -> Transform {
        Transform {
            m: mat,
            inv: mat.inverse()
        }
    }
}

/// Most implementations adapted from pbrt
impl Mat4 {
    pub fn new(t00: f64, t01: f64, t02: f64, t03: f64,
               t10: f64, t11: f64, t12: f64, t13: f64,
               t20: f64, t21: f64, t22: f64, t23: f64,
               t30: f64, t31: f64, t32: f64, t33: f64)
              -> Mat4 {

        let mut m = [[0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0]];

        m[0][0] = t00;
        m[0][1] = t01;
        m[0][2] = t02;
        m[0][3] = t03;

        m[1][0] = t10;
        m[1][1] = t11;
        m[1][2] = t12;
        m[1][3] = t13;

        m[2][0] = t20;
        m[2][1] = t21;
        m[2][2] = t22;
        m[2][3] = t23;

        m[3][0] = t30;
        m[3][1] = t31;
        m[3][2] = t32;
        m[3][3] = t33;

        Mat4 { m: m }
    }

    pub fn get(&self, row: usize, column: usize) -> f64 {
        self.m[row][column]
    }

    pub fn identity() -> Mat4 {
        Mat4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn zero() -> Mat4 {
        Mat4::new(
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0
        )
    }

    pub fn translate_matrix(v: &Vec3) -> Mat4 {
        Mat4::new(
            1.0, 0.0, 0.0, v.x,
            0.0, 1.0, 0.0, v.y,
            0.0, 0.0, 1.0, v.z,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn scale_matrix(v: &Vec3) -> Mat4 {
        Mat4::new(
            v.x, 0.0, 0.0, 0.0,
            0.0, v.y, 0.0, 0.0,
            0.0, 0.0, v.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn has_scale(&self) -> bool {
        Mat4::approx_eq(self.get(0, 0), self.get(1, 1)) &&
        Mat4::approx_eq(self.get(0, 0), self.get(2, 2))
    }

    /// This assumes the matrix is scalar; check has_scale(&self) -> bool before use
    pub fn scale(&self) -> f64 {
        self.m[0][0]
    }

    pub fn rotate_x_deg_matrix(angle: f64) -> Mat4 {
        let sin_t = Mat4::deg_to_rad(angle).sin();
        let cos_t = Mat4::deg_to_rad(angle).cos();

        Mat4::new(
            1.0,   0.0,    0.0, 0.0,
            0.0, cos_t, -sin_t, 0.0,
            0.0, sin_t,  cos_t, 0.0,
            0.0,   0.0,    0.0, 1.0
        )
    }

    pub fn rotate_y_deg_matrix(angle: f64) -> Mat4 {
        let sin_t = Mat4::deg_to_rad(angle).sin();
        let cos_t = Mat4::deg_to_rad(angle).cos();

        Mat4::new(
             cos_t, 0.0, sin_t, 0.0,
               0.0, 1.0,   0.0, 0.0,
            -sin_t, 0.0, cos_t, 0.0,
               0.0, 0.0,   0.0, 1.0
        )
    }

    pub fn rotate_z_deg_matrix(angle: f64) -> Mat4 {
        let sin_t = Mat4::deg_to_rad(angle).sin();
        let cos_t = Mat4::deg_to_rad(angle).cos();

        Mat4::new(
            cos_t, -sin_t, 0.0, 0.0,
            sin_t,  cos_t, 0.0, 0.0,
              0.0,    0.0, 1.0, 0.0,
              0.0,    0.0, 0.0, 1.0
        )
    }

    pub fn rotate_axis_deg_matrix(angle: f64, axis: &Vec3) -> Mat4 {
        let a = axis.unit();
        let s = Mat4::deg_to_rad(angle).sin();
        let c = Mat4::deg_to_rad(angle).cos();

        let mut m = [[0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0]];

        m[0][0] = a.x * a.x + (1.0 - a.x * a.x) * c;
        m[0][1] = a.x * a.y * (1.0 - c) - a.z * s;
        m[0][2] = a.x * a.z * (1.0 - c) + a.y * s;
        m[0][3] = 0.0;

        m[1][0] = a.x * a.y * (1.0 - c) + a.z * s;
        m[1][1] = a.y * a.y + (1.0 - a.y * a.y) * c;
        m[1][2] = a.y * a.z * (1.0 - c) - a.x * s;
        m[1][3] = 0.0;

        m[2][0] = a.x * a.z * (1.0 - c) - a.y * s;
        m[2][1] = a.y * a.z * (1.0 - c) + a.x * s;
        m[2][2] = a.z * a.z + (1.0 - a.z * a.z) * c;
        m[2][3] = 0.0;

        m[3][0] = 0.0;
        m[3][1] = 0.0;
        m[3][2] = 0.0;
        m[3][3] = 1.0;

        Mat4 { m: m }
    }

    /// This matrix translates between world-space and camera-space
    pub fn look_at_matrix(pos: &Vec3, up: &Vec3, look_at: &Vec3) -> Mat4 {
        let dir = (*look_at - *pos).unit();
        let left = (up.unit().cross(&dir)).unit();
        let new_up = dir.cross(&left);

        Mat4::new(
            left.x, new_up.x, dir.x, pos.x,
            left.y, new_up.y, dir.y, pos.y,
            left.z, new_up.z, dir.z, pos.z,
               0.0,      0.0,   0.0,   1.0
        )
    }

    pub fn transpose(&self) -> Mat4 {
        Mat4::new(
            self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0],
            self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1],
            self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2],
            self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]
        )
    }

    /// Normals cannot have the transformation matrix directly applied to them
    pub fn transform_normal(n: &Vec3, transform: &Mat4) -> Vec3 {
        let inv = transform.inverse();

        Vec3 {
            x: inv.m[0][0] * n.x + inv.m[1][0] * n.y + inv.m[2][0] * n.z,
            y: inv.m[0][1] * n.x + inv.m[1][1] * n.y + inv.m[2][1] * n.z,
            z: inv.m[0][2] * n.x + inv.m[1][2] * n.y + inv.m[2][2] * n.z
        }
    }

    pub fn transform_ray(_r: &Ray) -> Ray {
        panic!("Ray transform not implemented");
    }

    pub fn transform_bbox(_bbox: &BBox) -> BBox {
        panic!("BBox transform not implemented");
    }

    pub fn inverse(&self) -> Mat4 {
        let s0 = self.m[0][0] * self.m[1][1] - self.m[1][0] * self.m[0][1];
        let s1 = self.m[0][0] * self.m[1][2] - self.m[1][0] * self.m[0][2];
        let s2 = self.m[0][0] * self.m[1][3] - self.m[1][0] * self.m[0][3];
        let s3 = self.m[0][1] * self.m[1][2] - self.m[1][1] * self.m[0][2];
        let s4 = self.m[0][1] * self.m[1][3] - self.m[1][1] * self.m[0][3];
        let s5 = self.m[0][2] * self.m[1][3] - self.m[1][2] * self.m[0][3];

        let c5 = self.m[2][2] * self.m[3][3] - self.m[3][2] * self.m[2][3];
        let c4 = self.m[2][1] * self.m[3][3] - self.m[3][1] * self.m[2][3];
        let c3 = self.m[2][1] * self.m[3][2] - self.m[3][1] * self.m[2][2];
        let c2 = self.m[2][0] * self.m[3][3] - self.m[3][0] * self.m[2][3];
        let c1 = self.m[2][0] * self.m[3][2] - self.m[3][0] * self.m[2][2];
        let c0 = self.m[2][0] * self.m[3][1] - self.m[3][0] * self.m[2][1];

        let invdet = 1.0 / (s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0);

        let mut m = [[0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0, 0.0]];

        let a = self.m;

        m[0][0] = ( a[1][1] * c5 - a[1][2] * c4 + a[1][3] * c3) * invdet;
        m[0][1] = (-a[0][1] * c5 + a[0][2] * c4 - a[0][3] * c3) * invdet;
        m[0][2] = ( a[3][1] * s5 - a[3][2] * s4 + a[3][3] * s3) * invdet;
        m[0][3] = (-a[2][1] * s5 + a[2][2] * s4 - a[2][3] * s3) * invdet;

        m[1][0] = (-a[1][0] * c5 + a[1][2] * c2 - a[1][3] * c1) * invdet;
        m[1][1] = ( a[0][0] * c5 - a[0][2] * c2 + a[0][3] * c1) * invdet;
        m[1][2] = (-a[3][0] * s5 + a[3][2] * s2 - a[3][3] * s1) * invdet;
        m[1][3] = ( a[2][0] * s5 - a[2][2] * s2 + a[2][3] * s1) * invdet;

        m[2][0] = ( a[1][0] * c4 - a[1][1] * c2 + a[1][3] * c0) * invdet;
        m[2][1] = (-a[0][0] * c4 + a[0][1] * c2 - a[0][3] * c0) * invdet;
        m[2][2] = ( a[3][0] * s4 - a[3][1] * s2 + a[3][3] * s0) * invdet;
        m[2][3] = (-a[2][0] * s4 + a[2][1] * s2 - a[2][3] * s0) * invdet;

        m[3][0] = (-a[1][0] * c3 + a[1][1] * c1 - a[1][2] * c0) * invdet;
        m[3][1] = ( a[0][0] * c3 - a[0][1] * c1 + a[0][2] * c0) * invdet;
        m[3][2] = (-a[3][0] * s3 + a[3][1] * s1 - a[3][2] * s0) * invdet;
        m[3][3] = ( a[2][0] * s3 - a[2][1] * s1 + a[2][2] * s0) * invdet;

        Mat4 { m: m }
    }

    /// http://csherratt.github.io/csherratt/blog/2013/11/24/matrix-multiply-in-rust/
    /// Note: This is the slow, unoptimised version!
    pub fn mult_m(a: &Mat4, b: &Mat4) -> Mat4 {
        let mut out = Mat4 {
            m: [[0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0]]
        };

        for i in 0usize..4 {
            for j in 0usize..4 {
                for k in 0usize..4 {
                    out.m[i][j] += a.m[i][k] * b.m[k][j];
                }
            }
        }

        out
    }

    pub fn mult_v(m: &Mat4, v: &Vec3) -> Vec3 {
        Vec3 {
            x: m.m[0][0] * v.x + m.m[0][1] * v.y + m.m[0][2] * v.z,
            y: m.m[1][0] * v.x + m.m[1][1] * v.y + m.m[1][2] * v.z,
            z: m.m[2][0] * v.x + m.m[2][1] * v.y + m.m[2][2] * v.z
        }
    }

    pub fn mult_p(m: &Mat4, p: &Vec3) -> Vec3 {
        let xp = m.m[0][0] * p.x + m.m[0][1] * p.y + m.m[0][2] * p.z + m.m[0][3];
        let yp = m.m[1][0] * p.x + m.m[1][1] * p.y + m.m[1][2] * p.z + m.m[1][3];
        let zp = m.m[2][0] * p.x + m.m[2][1] * p.y + m.m[2][2] * p.z + m.m[2][3];
        let wp = m.m[3][0] * p.x + m.m[3][1] * p.y + m.m[3][2] * p.z + m.m[3][3];

        if wp == 1.0 {
            // Optimisation, wp == 1.0 is common
            Vec3 {
                x: xp,
                y: yp,
                z: zp
            }
        } else {
            // Perspective division
            Vec3 {
                x: xp / wp,
                y: yp / wp,
                z: zp / wp
            }
        }
    }

    fn approx_eq(f1: f64, f2: f64) -> bool {
        (f1 - f2).abs() < ::std::f64::EPSILON
    }

    fn deg_to_rad(deg: f64) -> f64 {
        deg * PI / 180.0
    }
}

impl cmp::PartialEq for Mat4 {
    fn eq(&self, other: &Mat4) -> bool {
        self.m[0][0] == other.m[0][0] &&
        self.m[0][1] == other.m[0][1] &&
        self.m[0][2] == other.m[0][2] &&
        self.m[0][3] == other.m[0][3] &&

        self.m[1][0] == other.m[1][0] &&
        self.m[1][1] == other.m[1][1] &&
        self.m[1][2] == other.m[1][2] &&
        self.m[1][3] == other.m[1][3] &&

        self.m[2][0] == other.m[2][0] &&
        self.m[2][1] == other.m[2][1] &&
        self.m[2][2] == other.m[2][2] &&
        self.m[2][3] == other.m[2][3] &&

        self.m[3][0] == other.m[3][0] &&
        self.m[3][1] == other.m[3][1] &&
        self.m[3][2] == other.m[3][2] &&
        self.m[3][3] == other.m[3][3]
    }

    fn ne(&self, other: &Mat4) -> bool {
        !(self.eq(other))
    }
}

impl Add for Mat4 {
    type Output = Mat4;

    fn add(self, other: Mat4) -> Mat4 {
        let mut out = Mat4 {
            m: [[0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0]]
        };

        out.m[0][0] = self.m[0][0] + other.m[0][0];
        out.m[0][1] = self.m[0][1] + other.m[0][1];
        out.m[0][2] = self.m[0][2] + other.m[0][2];
        out.m[0][3] = self.m[0][3] + other.m[0][3];

        out.m[1][0] = self.m[1][0] + other.m[1][0];
        out.m[1][1] = self.m[1][1] + other.m[1][1];
        out.m[1][2] = self.m[1][2] + other.m[1][2];
        out.m[1][3] = self.m[1][3] + other.m[1][3];

        out.m[2][0] = self.m[2][0] + other.m[2][0];
        out.m[2][1] = self.m[2][1] + other.m[2][1];
        out.m[2][2] = self.m[2][2] + other.m[2][2];
        out.m[2][3] = self.m[2][3] + other.m[2][3];

        out.m[3][0] = self.m[3][0] + other.m[3][0];
        out.m[3][1] = self.m[3][1] + other.m[3][1];
        out.m[3][2] = self.m[3][2] + other.m[3][2];
        out.m[3][3] = self.m[3][3] + other.m[3][3];

        out
    }
}

impl Sub for Mat4 {
    type Output = Mat4;

    fn sub(self, other: Mat4) -> Mat4 {
        let mut out = Mat4 {
            m: [[0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0]]
        };

        out.m[0][0] = self.m[0][0] - other.m[0][0];
        out.m[0][1] = self.m[0][1] - other.m[0][1];
        out.m[0][2] = self.m[0][2] - other.m[0][2];
        out.m[0][3] = self.m[0][3] - other.m[0][3];

        out.m[1][0] = self.m[1][0] - other.m[1][0];
        out.m[1][1] = self.m[1][1] - other.m[1][1];
        out.m[1][2] = self.m[1][2] - other.m[1][2];
        out.m[1][3] = self.m[1][3] - other.m[1][3];

        out.m[2][0] = self.m[2][0] - other.m[2][0];
        out.m[2][1] = self.m[2][1] - other.m[2][1];
        out.m[2][2] = self.m[2][2] - other.m[2][2];
        out.m[2][3] = self.m[2][3] - other.m[2][3];

        out.m[3][0] = self.m[3][0] - other.m[3][0];
        out.m[3][1] = self.m[3][1] - other.m[3][1];
        out.m[3][2] = self.m[3][2] - other.m[3][2];
        out.m[3][3] = self.m[3][3] - other.m[3][3];

        out
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        Mat4::mult_m(&self, &other)
    }
}

impl fmt::Debug for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 46 spaces in between
        write!(f,
            "\n┌                                              ┐\n\
               │{: >10.3}, {: >10.3}, {: >10.3}, {: >10.3}│\n\
               │{: >10.3}, {: >10.3}, {: >10.3}, {: >10.3}│\n\
               │{: >10.3}, {: >10.3}, {: >10.3}, {: >10.3}│\n\
               │{: >10.3}, {: >10.3}, {: >10.3}, {: >10.3}│\n\
               └                                              ┘\n",
               self.m[0][0], self.m[0][1], self.m[0][2], self.m[0][3],
               self.m[1][0], self.m[1][1], self.m[1][2], self.m[1][3],
               self.m[2][0], self.m[2][1], self.m[2][2], self.m[2][3],
               self.m[3][0], self.m[3][1], self.m[3][2], self.m[3][3]
        )
    }
}

#[test]
fn test_add() {
    let m = Mat4::new(
         1.0,  2.0,  3.0,  4.0,
         5.0,  6.0,  7.0,  8.0,
         9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0
    );

    let expected = Mat4::new(
         2.0,  4.0,  6.0,  8.0,
        10.0, 12.0, 14.0, 16.0,
        18.0, 20.0, 22.0, 24.0,
        26.0, 28.0, 30.0, 32.0
    );

    assert_eq!(m + m, expected);
}

#[test]
fn test_sub() {
    let m = Mat4::new(
         1.0,  2.0,  3.0,  4.0,
         5.0,  6.0,  7.0,  8.0,
         9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0
    );

    assert_eq!(m - m, Mat4::zero());
}

#[test]
fn test_mul() {
    let a = Mat4::new(
         1.0,  3.0,  5.0,  7.0,
        11.0, 13.0, 17.0, 23.0,
        29.0, 31.0, 37.0, 41.0,
        43.0, 47.0, 53.0, 59.0
    );

    let b = Mat4::new(
         1.0,  2.0,  3.0,  4.0,
         5.0,  6.0,  7.0,  8.0,
         9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0
    );

    let expected = Mat4::new(
         152.0,  168.0,  184.0,  200.0,
         528.0,  592.0,  656.0,  720.0,
        1050.0, 1188.0, 1326.0, 1464.0,
        1522.0, 1724.0, 1926.0, 2128.0
    );

    let out = Mat4::mult_m(&a, &b);
    assert_eq!(out, expected);
}

#[test]
fn test_equality() {
    let i1 = Mat4::identity();
    let i2 = Mat4::identity();
    let zero = Mat4::zero();

    assert!(i1 == i2);
    assert!(i1 != zero);
}

#[test]
fn test_inverse() {
    let i = Mat4::identity();
    assert_eq!(i, i.inverse());

    let m = Mat4::new(
        1.0, 0.0, 1.0, 1.0,
        2.0, 0.0, 1.0, 0.0,
        2.0, 1.0, 1.0, 0.0,
        0.0, 0.0, 1.0, 3.0
    );

    let m_inverse = Mat4::new(
        -3.0,  2.0, 0.0,  1.0,
         0.0, -1.0, 1.0,  0.0,
         6.0, -3.0, 0.0, -2.0,
        -2.0,  1.0, 0.0,  1.0
    );

    assert_eq!(m.inverse(), m_inverse);
}

#[test]
fn test_transpose() {
    let m = Mat4::new(
         1.0,  2.0,  3.0,  4.0,
         5.0,  6.0,  7.0,  8.0,
         9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0
    );

    let mt = Mat4::new(
        1.0,  5.0,  9.0, 13.0,
        2.0,  6.0, 10.0, 14.0,
        3.0,  7.0, 11.0, 15.0,
        4.0,  8.0, 12.0, 16.0
    );

    assert!(m.transpose() == mt);
}

#[test]
fn test_mul_with_vec() {
    let m = Mat4::new(
         1.0,  2.0,  3.0,  4.0,
         5.0,  6.0,  7.0,  8.0,
         9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0
    );

    let v = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0
    };

    let expected_w0 = Vec3 {
        x: 1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0,
        y: 5.0 * 1.0 + 6.0 * 2.0 + 7.0 * 3.0,
        z: 9.0 * 1.0 + 10.0 * 2.0 + 11.0 * 3.0
    };

    let multiplied_w0 = Mat4::mult_v(&m, &v);
    assert_eq!(multiplied_w0, expected_w0);
}
