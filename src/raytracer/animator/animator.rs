use raytracer::animator::CameraKeyframe;
use raytracer::Renderer;
use scene::{Camera, Scene};
use std::sync::mpsc::sync_channel;
use std::sync::Arc;
use std::thread;
use vec3::Vec3;

pub struct Animator {
    pub fps: f64,
    pub animate_from: f64, // Number of frames is rounded down to nearest frame
    pub animate_to: f64,
    pub starting_frame_number: u32, // For filename
    pub renderer: Renderer
}

// TODO: Non-linear interpolation
impl Animator {
    // TODO: make this a Surface iterator so both single frame and animation
    // process flows are similar
    pub fn animate(&self, camera: Camera, shared_scene: Arc<Scene>, filename: &str) {
        let animate_start = ::time::get_time();
        let length = self.animate_to - self.animate_from;
        let total_frames = (self.fps * length).floor() as u32;

        // Allow one frame to be renderered while the previous one is being written
        let (frame_tx, frame_rx) = sync_channel(0);
        let (exit_tx, exit_rx) = sync_channel(0);

        let starting_frame_number = self.starting_frame_number;

        let filename = filename.to_string();
        thread::spawn(move || {
            for (frame_num, frame_data) in frame_rx.iter().enumerate() {
                let file_frame_number = starting_frame_number as usize + frame_num;

                let shared_name = format!("{}{:06}.ppm", filename, file_frame_number);
                ::util::export::to_ppm(frame_data, &shared_name);
            }

            exit_tx.send(()).unwrap();
        });

        for frame_number in 0..total_frames {
            let time = self.animate_from + frame_number as f64 / self.fps;
            let lerped_camera = Animator::lerp_camera(&camera, time);
            let frame_data = self.renderer.render(lerped_camera, shared_scene.clone());
            frame_tx.send(frame_data).unwrap();

            ::util::print_progress("*** Frame", animate_start.clone(), frame_number as usize + 1usize, total_frames as usize);
            println!("");
        }
        drop(frame_tx);

        let () = exit_rx.recv().unwrap();
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

        let alpha = if keyframe_length == 0.0 {
            0.0
        } else {
            second.easing.t((time - first.time) / keyframe_length)
        };

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

#[cfg(test)]
use raytracer::animator::Easing;

#[test]
fn test_lerp_camera_position() {
    // Camera rotates 180 degrees
    let camera = Camera::new_with_keyframes(
        Vec3 { x: -1.0, y: -1.0, z: -1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        45.0,
        10,
        10,
        vec![
            CameraKeyframe {
                time: 5.0,
                position: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                look_at: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                easing: Easing::linear()
            },
            CameraKeyframe {
                time: 10.0,
                position: Vec3 { x: 10.0, y: 0.0, z: 0.0 },
                look_at: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                easing: Easing::linear()
            },
        ]
    );

    let expected_position_0 = Vec3 { x: -1.0, y: -1.0, z: -1.0 };
    assert_eq!(Animator::lerp_camera(&camera, 0.0).position, expected_position_0);

    let expected_position_5 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    assert_eq!(Animator::lerp_camera(&camera, 5.0).position, expected_position_5);

    let expected_position_7_5 = Vec3 { x: 5.0, y: 0.0, z: 0.0 };
    assert_eq!(Animator::lerp_camera(&camera, 7.5).position, expected_position_7_5);

    let expected_position_10 = Vec3 { x: 10.0, y: 0.0, z: 0.0 };
    assert_eq!(Animator::lerp_camera(&camera, 10.0).position, expected_position_10);
}
