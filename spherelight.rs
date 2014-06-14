use light::Light;
use vec3::Vec3;
use std::rand::{task_rng, Rng};

pub struct SphereLight {
    pub position: Vec3,
    pub color: Vec3,
    pub radius: f64
}

impl Light for SphereLight {
    fn position(&self) -> Vec3 {
        let mut rng = task_rng();
        let j_x: f64 = rng.gen();
        let j_y: f64 = rng.gen();
        let j_z: f64 = rng.gen();

        let jitter = Vec3 {
            x: self.radius * (j_x - 0.5),
            y: self.radius * (j_y - 0.5),
            z: self.radius * (j_z - 0.5)
        };

        self.position + jitter
    }

    fn color(&self) -> Vec3 {
        self.color
    }

    fn center(&self) -> Vec3 {
        self.position
    }

    fn is_point(&self) -> bool {
        false
    }
}
