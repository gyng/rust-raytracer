use raytracer::animator::CameraKeyframe;
use raytracer::Renderer;
use scene::{Camera, Scene};
use std::sync::Arc;
use vec3::Vec3;


pub struct Animator {
    pub fps: f64,
    pub length: f64, // rounded down to nearest frame
    pub renderer: Renderer
}


// TODO: Non-linear interpolation
// TODO: Improve keyframes (sort/order them as we don't need dynamic keyframe insertion)
impl Animator {
    // TODO: make this a Surface iterator so both single frame and animation
    // process flows are similar
    pub fn animate(&self, camera: Camera, shared_scene: Arc<Scene>, filename: &str) {
        let start_time = ::time::get_time();
        let total_frames = (self.fps * self.length).floor() as uint;

        for frame_number in range(0, total_frames) {
            let time = frame_number as f64 / self.fps;
            let lerped_camera = Animator::lerp_camera(&camera, time);
            let frame_data = self.renderer.render(lerped_camera, shared_scene.clone());

            ::util::export::to_ppm(frame_data, format!("{}{:06u}.ppm", filename, frame_number).as_slice());
            ::util::print_progress("*** Frame", start_time, frame_number as uint, total_frames);
            println!("");
        }
    }

    fn get_neighbour_keyframes(keyframes: Vec<CameraKeyframe>, time: f64)
                               -> (CameraKeyframe, CameraKeyframe, f64) {

        if keyframes.len() <= 1 {
            fail!("Not enough keyframes to interpolate: got: {} expected: >= 2", keyframes.len());
        }

        // Get the two keyframes inbetween current time
        let mut first = &keyframes[0];
        let mut second = &keyframes[1];

        for keyframe in keyframes.iter() {
            if keyframe.time <= time &&
               time - keyframe.time <= time - first.time {
                first = keyframe;
            }

            if keyframe.time > time &&
               keyframe.time - time < second.time - time {
                second = keyframe;
            }
        }

        let keyframe_length = second.time - first.time;
        let alpha = (time - first.time) / keyframe_length;

        (first.clone(), second.clone(), alpha)
    }

    fn lerp_camera(camera: &Camera, time: f64) -> Camera {
        let keyframes = match camera.keyframes.clone() {
            Some(k) => k,
            None => fail!("Cannot lerp a camera with no keyframes!")
        };

        let (first, second, alpha) = Animator::get_neighbour_keyframes(keyframes, time);

        let lerped_position = Vec3::lerp(&first.position, &second.position, alpha);
        let lerped_look_at  = Vec3::lerp(&first.look_at, &second.look_at, alpha);
        let lerped_up       = Vec3::lerp(&first.up, &second.up, alpha);

        let mut lerped_camera = Camera::new(
            lerped_position,
            lerped_look_at,
            lerped_up,
            camera.fov_deg,
            camera.image_width,
            camera.image_height,
        );

        lerped_camera.keyframes = camera.keyframes.clone();
        lerped_camera
    }
}
