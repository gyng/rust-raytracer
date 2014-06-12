pub mod cow {
    pub struct Vec3 {
        x: f64,
        y: f64,
        z: f64
    }

    impl Vec3 {
        fn len(&self) -> f64 {
            (self.x * self.x +
             self.y * self.y +
             self.z * self.z).sqrt()
        }

        fn dot(&self, other: &Vec3) -> f64 {
            self.x * other.x +
            self.y * other.y +
            self.z * other.z
        }

        fn cross(&self, other: &Vec3) -> Vec3 {
            Vec3 {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x
            }
        }

        fn unit(&self) -> Vec3 {
            let len = self.len();

            Vec3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len
            }
        }
    }

    impl Add<Vec3, Vec3> for Vec3 {
        fn add(&self, other: &Vec3) -> Vec3 {
            Vec3 {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z
            }
        }
    }

    impl Sub<Vec3, Vec3> for Vec3 {
        fn sub(&self, other: &Vec3) -> Vec3 {
            Vec3 {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z
            }
        }
    }

    impl Mul<Vec3, Vec3> for Vec3 {
        fn mul(&self, other: &Vec3) -> Vec3 {
            Vec3 {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z
            }
        }
    }
}
