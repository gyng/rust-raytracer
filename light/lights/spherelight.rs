use std::rand::{task_rng, Rng};
use light::light::Light;
use vec3::Vec3;

#[allow(dead_code)]
pub struct SphereLight {
    pub position: Vec3,
    pub color: Vec3,
    pub radius: f64
}

impl Light for SphereLight {
    fn position(&self) -> Vec3 {
        let mut rng = task_rng();

        let jitter = Vec3 {
            x: self.radius * (rng.gen::<f64>() - 0.5),
            y: self.radius * (rng.gen::<f64>() - 0.5),
            z: self.radius * (rng.gen::<f64>() - 0.5)
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
