use raytracer::animator::CameraKeyframe;
use raytracer::Renderer;
use scene::{Camera, Scene};
use std::num::Float;
use std::sync::{Arc, Semaphore};
use std::thread::Thread;
use vec3::Vec3;

pub struct Animator {
    pub fps: f64,
    pub animate_from: f64, // Number of frames is rounded down to nearest frame
    pub animate_to: f64,
    pub starting_frame_number: uint, // For filename
    pub renderer: Renderer
}

// TODO: Non-linear interpolation
impl Animator {
    // TODO: make this a Surface iterator so both single frame and animation
    // process flows are similar
    pub fn animate(&self, camera: Camera, shared_scene: Arc<Scene>, filename: &str) {
        let animate_start = ::time::get_time();
        let length = self.animate_to - self.animate_from;
        let total_frames = (self.fps * length).floor() as uint;

        let sema = Arc::new(Semaphore::new(1));

        for frame_number in range(0, total_frames) {
            let time = self.animate_from + frame_number as f64 / self.fps;
            let lerped_camera = Animator::lerp_camera(&camera, time);
            let frame_data = self.renderer.render(lerped_camera, shared_scene.clone());

            let file_frame_number = self.starting_frame_number + frame_number;
            let shared_name = format!("{}{:06}.ppm", filename, file_frame_number);

            let child_sema = sema.clone();
            sema.acquire();

            // Continue animating next frame as writing rendered frame to disk (slow) occurs
            Thread::spawn(move || {
                ::util::export::to_ppm(frame_data, shared_name[]);
                child_sema.release();
            }).detach();

            ::util::print_progress("*** Frame", animate_start.clone(), frame_number + 1u, total_frames);
            println!("");
        }

        sema.acquire();
    }

    fn get_neighbour_keyframes(keyframes: Vec<CameraKeyframe>, time: f64)
                               -> (CameraKeyframe, CameraKeyframe, f64) {

        if keyframes.len() <= 1 {
            panic!("Not enough keyframes to interpolate: got: {} expected: >= 2", keyframes.len());
        }

        // Get the two keyframes inbetween current time
        let mut first = &keyframes[0];
        let mut second = &keyframes[1];

        for keyframe in keyframes.iter() {
            if keyframe.time <= time && time - keyframe.time >= first.time - time {
                first = keyframe;
            }

            if keyframe.time > time &&
               (keyframe.time - time < second.time - time || second.time < time) {
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
            None => panic!("Cannot lerp a camera with no keyframes!")
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
