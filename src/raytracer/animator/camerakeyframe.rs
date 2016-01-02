use vec3::Vec3;
use raytracer::animator::Easing;

#[derive(Clone)]
pub struct CameraKeyframe {
    pub time: f64,
    pub position: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub easing: Easing
}
