use vec3::Vec3;
use ray::Ray;

pub struct Camera {
    pub position: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub fov_deg: f64,
    pub image_width: int,
    pub image_height: int,
    eye: Vec3,
    right: Vec3,
    half_width: f64,
    half_height: f64,
    pixel_width: f64,
    pixel_height: f64
}

impl Camera {
    pub fn set_position(&mut self, new_position: Vec3) -> () {
        self.position = new_position;
        self.update_eye_vector();
    }

    pub fn set_look_at(&mut self, new_look_at: Vec3) -> () {
        self.look_at = new_look_at;
        self.update_eye_vector();
    }

    pub fn set_fov(&mut self, new_fov_deg: f64) -> () {
        self.fov_deg = new_fov_deg;
        self.update_internal_sizes();
    }

    pub fn set_image_size(&mut self, width: int, height: int) -> () {
        self.image_width = width;
        self.image_height = height;
        self.update_internal_sizes();
    }

    pub fn get_ray(&self, x: int, y: int) -> Ray {
        Ray {
            origin: self.position,
            direction: self.eye +
                self.right.scale(x as f64 * self.pixel_width - self.half_width) +
                self.up.scale(y as f64 * self.pixel_width - self.half_height)
        }
    }

    fn update_eye_vector(&mut self) -> () {
        self.eye = (self.look_at - self.position).unit();
        self.right = self.eye.cross(&self.up);
    }

    fn update_internal_sizes(&mut self) -> () {
        let fov_rad = self.fov_deg.to_radians();
        let ratio = self.image_height as f64 / self.image_width as f64;

        self.half_width  = fov_rad.tan();
        self.half_height = self.half_width * ratio;

        let camera_width  = self.half_width  * 2.0;
        let camera_height = self.half_height * 2.0;

        self.pixel_width  = camera_width  / (self.image_width  - 1) as f64;
        self.pixel_height = camera_height / (self.image_height - 1) as f64;
    }
}
