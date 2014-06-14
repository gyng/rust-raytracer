/// Vec3 is three words: should parameters to functions be references or values?
/// My understanding is that structs of 1 (or 2?) words should be passed by value,
/// while larger structs should be passed by reference.
#[deriving(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn one() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x +
         self.y * self.y +
         self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn unit(&self) -> Vec3 {
        let len = self.len();

        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        }
    }

    pub fn scale(&self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }

    /// V, N should be unit vectors
    //
    //  ^  ^
    // V \ | N
    //    \|
    // =========
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        n.scale(2.0 * (n.dot(v))) - *v
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
