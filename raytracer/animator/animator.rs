use raytracer::Renderer;
use scene::{Camera, Scene};
use vec3::Vec3;


pub struct Animator {
    pub fps: f64,
    pub length: f64, // rounded down to nearest frame
    pub renderer: Renderer
}


/// TODO: non-linear interpolation, bring keyframe search out into function
impl Animator {
    pub fn animate(&self, camera: Camera, scene: Scene, filename: &str) {
        let start_time = ::time::get_time();
        let total_frames = (self.fps * self.length).floor() as uint;

        for frame_number in range(0, total_frames) {
            ::util::print_progress("*** Frame", start_time, frame_number as uint, total_frames);
            let time = frame_number as f64 * self.fps;
            let lerped_camera = Animator::lerp_camera(&camera, time);
            let frame_data = self.renderer.render(lerped_camera, scene);

            ::util::export::to_ppm(frame_data, format!("{}{}", filename, frame_number).as_slice());
        }

        ::util::print_progress("*** Frame", start_time, total_frames, total_frames);
    }

    fn lerp_camera(camera: &Camera, current_time: f64) -> Camera {
        let keyframes = match camera.keyframes.clone() {
            Some(k) => {
                if k.len() <= 1 {
                    fail!("Not enough camera keyframes: got: {} expected: >= 2", k.len());
                }
                k
            },
            None => fail!("Cannot lerp a camera with no keyframes!")
        };

        // Get the two keyframes inbetween current time
        let mut first = &keyframes[0];
        let mut second = &keyframes[1];

        for keyframe in keyframes.iter() {
            if keyframe.time <= current_time &&
               current_time - keyframe.time <= current_time - first.time {
                first = keyframe;
            }

            if keyframe.time > current_time &&
               keyframe.time - current_time < second.time - current_time {
                second = keyframe;
            }
        }

        let keyframe_length = second.time - first.time;
        let alpha = (current_time - first.time) / keyframe_length;

        // TODO: move lerp into vec3
        let lerped_position = first.position + (second.position - first.position).scale(alpha);
        let lerped_look_at = first.look_at + (second.look_at - first.look_at).scale(alpha);
        let lerped_up = first.up + (second.up - first.up).scale(alpha);

        let mut lerped_camera = Camera::new(
            lerped_position,
            lerped_look_at,
            lerped_up,
            camera.fov_deg,
            camera.image_width,
            camera.image_height,
        );

        lerped_camera.keyframes = camera.keyframes;
        lerped_camera
    }
}
