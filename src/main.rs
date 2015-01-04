#![feature(associated_types)]
#![feature(macro_rules)]
#![feature(slicing_syntax)]
#![feature(old_orphan_check)]
#![deny(unused_imports)]

extern crate time;
extern crate "rustc-serialize" as rustc_serialize;

use scene::{Camera, Scene};

use std::io::File;
use std::io;
use std::os;
use std::sync::Arc;
use rustc_serialize::json;
use rustc_serialize::json::DecoderError::MissingFieldError;

mod geometry;
mod light;
mod material;
mod my_scene;
mod raytracer;
mod scene;
mod util;
mod vec3;
mod mat4;

// Replace this with argparse eventually
struct ProgramArgs {
    config_file: String
}

#[derive(RustcDecodable, RustcEncodable)]
struct SceneConfig<'a> {
    name: String,
    size: (int, int),
    fov: f64,
    reflect_depth: uint,
    refract_depth: uint,
    shadow_samples: uint,
    pixel_samples: uint,
    output_file: String,
    animating: bool,
    fps: f64,
    time_slice: (f64, f64),
    starting_frame_number: uint
}

fn parse_args(args: Vec<String>) -> Result<ProgramArgs, String>  {
    let (program_name, rest) = match args[] {
        // I wouldn't expect this in the wild
        [] => panic!("Args do not even include a program name"),
        [ref program_name, rest..] => (
            program_name,
            rest
        )
    };
    match rest.len() {
        0 => Err(format!("Usage: {} scene_config.json", program_name)),
        1 => Ok(ProgramArgs {
            config_file: rest[0].clone()
        }),
        _ => Err(format!("Usage: {} scene_config.json", program_name)),
    }
}

fn get_camera_and_scene(config: &SceneConfig) -> Option<(Camera, Scene)> {
    let scene_name = config.name.clone();
    let (image_width, image_height) = config.size;
    let fov = config.fov;

    // Cameras, scenes created in ./my_scene.rs
    // Scenes with an octree supplied (see my_scene.rs) will use it.
    // Lower the render quality (especially shadow_samples) for complex scenes
    return match scene_name[] {
        "box" => {
            // Box. Simplest scene with 9 primitives, no octree
            let camera = my_scene::cornell::get_camera(image_width, image_height, fov);
            let scene = my_scene::cornell::get_scene();
            Some((camera, scene))
        },
        "bunny" => {
            // Bunny. Around 300 primitives, 2 lights. Uses octree. Has skybox, textures are
            // in another repository.
            let camera = my_scene::bunny::get_camera(image_width, image_height, fov);
            let scene = my_scene::bunny::get_scene();
            Some((camera, scene))
        },
        "teapot" => {
            // Teapot. Around 2500 polygons. Octree helps a bit. Has skybox.
            let camera = my_scene::teapot::get_teapot_camera(image_width, image_height, fov);
            let scene = my_scene::teapot::get_teapot_scene();
            Some((camera, scene))
        },
        "cow" => {
            // Cow. Around 5000 polygons. Octree helps considerably.
            let camera = my_scene::cow::get_camera(image_width, image_height, fov);
            let scene = my_scene::cow::get_scene();
            Some((camera, scene))
        },
        "lucy" => {
            // Lucy. Around 525814+1 primitives. Octree pretty much required. The model is included
            // separately, in another repository. Has skybox.
            let camera = my_scene::lucy::get_camera(image_width, image_height, fov);
            let scene = my_scene::lucy::get_scene();
            Some((camera, scene))
        },
        "sponza" => {
            // Sponza. Around 28K triangles, but more complex than Lucy. 2 lights.
            let camera = my_scene::sponza::get_camera(image_width, image_height, fov);
            let scene = my_scene::sponza::get_scene();
            Some((camera, scene))
        },
        "sibenik" => {
            // Sibenik, around 70K triangles, no texture work, 3 lights.
            let camera = match config.animating {
                true => my_scene::sibenik::get_animation_camera(image_width, image_height, fov),
                false => my_scene::sibenik::get_camera(image_width, image_height, fov)
            };
            let scene = my_scene::sibenik::get_scene();
            Some((camera, scene))
        },
        "heptoroid-white" => {
            // Heptoroid, 114688 tris, 57302 verts
            let camera = my_scene::heptoroid::get_camera(image_width, image_height, fov);
            let scene = my_scene::heptoroid::get_scene("white");
            Some((camera, scene))
        },
        "heptoroid-shiny" => {
            // Shiny heptoroid, 114688 tris, 57302 verts
            let camera = my_scene::heptoroid::get_camera(image_width, image_height, fov);
            let scene = my_scene::heptoroid::get_scene("shiny");
            Some((camera, scene))
        },
        "heptoroid-refractive" => {
            // Refractive heptoroid, you want to limit your reflect levels (2/3?)
            // and up your refract levels (10/16?) for this
            let camera = my_scene::heptoroid::get_camera(image_width, image_height, fov);
            let scene = my_scene::heptoroid::get_scene("refractive");
            Some((camera, scene))
        },
        "tachikoma" => {
            // Shiny heptoroid, 114688 tris, 57302 verts
            // You can forget about refractions, it's too complex a scene
            let camera = my_scene::tachikoma::get_camera(image_width, image_height, fov);
            let scene = my_scene::tachikoma::get_scene();
            Some((camera, scene))
        },
        "sphere" => {
            // Sphere skybox test scene
            let camera = match config.animating {
                true => my_scene::sphere::get_animation_camera(image_width, image_height, fov),
                false => my_scene::sphere::get_camera(image_width, image_height, fov)
            };
            let scene = my_scene::sphere::get_scene();
            Some((camera, scene))
        },
        "fresnel" => {
            // Fresnel test scene
            let camera = match config.animating {
                true => my_scene::fresnel::get_animation_camera(image_width, image_height, fov),
                false => my_scene::fresnel::get_camera(image_width, image_height, fov)
            };
            let scene = my_scene::fresnel::get_scene();
            Some((camera, scene))
        },
        _ => None
    };
}

fn main() {
    let start_time = ::time::get_time().sec;

    let program_args = match parse_args(os::args()) {
        Ok(program_args) => program_args,
        Err(mut error_str) => {
            error_str.push_str("\n");
            let mut stderr = io::stderr();
            assert!(stderr.write(error_str.as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };
    let config_path = Path::new(program_args.config_file);
    let mut file_handle = match File::open(&config_path) {
        Ok(file) => file,
        Err(err) => {
            let mut stderr = io::stderr();
            assert!(stderr.write(format!("{}\n", err).as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };
    let json_data = match file_handle.read_to_string() {
        Ok(data) => data,
        Err(err) => {
            let mut stderr = io::stderr();
            assert!(stderr.write(format!("{}\n", err).as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };

    let config: SceneConfig = match json::decode(json_data[]) {
        Ok(data) => data,
        Err(err) => {
            let mut stderr = io::stderr();
            let msg = match err {
                MissingFieldError(field_name) => {
                    format!("parse failure, missing field ``{}''\n", field_name)
                },
                _ => {
                    format!("parse failure: {}", err)
                }
            };
            assert!(stderr.write(msg.as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };

    println!("Job started at {}...\nLoading scene...", start_time);

    let scenepair = get_camera_and_scene(&config);
    let (camera, scene) = match scenepair {
        Some(pair) => pair,
        None => {
            let mut stderr = io::stderr();
            let msg = format!("unknown scene ``{}''\n", config.name);
            assert!(stderr.write(msg.as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };

    let shared_scene = Arc::new(scene); // Hackish solution for animator

    let scene_time = ::time::get_time().sec;
    println!("Scene loaded at {} ({}s)...", scene_time, scene_time - start_time);

    let renderer = raytracer::Renderer {
        reflect_depth: config.reflect_depth,
        refract_depth: config.refract_depth,
        shadow_samples: config.shadow_samples,
        pixel_samples: config.pixel_samples,
        // Number of tasks to spawn. Will use up max available cores.
        tasks: std::os::num_cpus()
    };

    if config.animating {
        let (animate_from, animate_to) = config.time_slice;

        let animator = raytracer::animator::Animator {
            fps: config.fps,
            animate_from: animate_from,
            animate_to: animate_to,
            starting_frame_number: config.starting_frame_number,
            renderer: renderer
        };

        println!("Animating - tasks: {}, FPS: {}, start: {}s, end:{}s, starting frame: {}",
                 std::os::num_cpus(), animator.fps, animator.animate_from, animator.animate_to,
                 animator.starting_frame_number);
        animator.animate(camera, shared_scene, config.output_file[]);
        let render_time = ::time::get_time().sec;
        println!("Render done at {} ({}s)",
                 render_time, render_time - scene_time);
    } else {
        // Still frame
        println!("Rendering with {} tasks...", std::os::num_cpus());
        let image_data = renderer.render(camera, shared_scene);
        let render_time = ::time::get_time().sec;
        println!("Render done at {} ({}s)...\nWriting file...",
                 render_time, render_time - scene_time);

        let out_file = format!("{}{}", config.output_file[], ".ppm");
        util::export::to_ppm(image_data, out_file[]);
        let export_time = ::time::get_time().sec;

        println!("Write done: {} ({}s). Written to {}\nTotal: {}s",
                 export_time, export_time - render_time,
                 config.output_file[], export_time - start_time);
    }
}
