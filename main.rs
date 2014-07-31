extern crate time;
extern crate serialize;

use scene::{Camera, Scene};

use std::io;
use std::io::File;
use std::os;
use serialize::json;
use serialize::json::MissingFieldError;

use std::sync::Arc;

mod geometry;
mod light;
mod material;
mod my_scene;
mod raytracer;
mod scene;
mod util;
mod vec3;


// Replace this with argparse eventually
struct ProgramArgs {
    config_file: String
}


#[deriving(Decodable, Encodable)]
struct SceneConfig<'a> {
    name: String,
    size: (int, int),
    fov: f64,
    reflect_depth: uint,
    refract_depth: uint,
    shadow_samples: uint,
    pixel_samples: uint,
    output_file: String,
    animating: bool
}


fn parse_args(args: Vec<String>) -> Result<ProgramArgs, String>  {
    let (program_name, rest) = match args.as_slice() {
        // I wouldn't expect this in the wild
        [] => fail!("Args do not even include a program name"),
        [ref program_name, ..rest] => (
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
    return match scene_name.as_slice() {
        "box" => {
            // Box. Simplest scene with 9 primitives, no octree
            let camera = my_scene::get_camera(image_width, image_height, fov);
            let scene = my_scene::get_scene();
            Some((camera, scene))
        },
        "bunny" => {
            // Bunny. Around 300 primitives, 2 lights. Uses octree. Has skybox, textures are
            // in another repository.
            let camera = my_scene::get_bunny_camera(image_width, image_height, fov);
            let scene = my_scene::get_bunny_scene();
            Some((camera, scene))
        },
        "teapot" => {
            // Teapot. Around 2500 polygons. Octree helps a bit. Has skybox.
            let camera = my_scene::get_teapot_camera(image_width, image_height, fov);
            let scene = my_scene::get_teapot_scene();
            Some((camera, scene))
        },
        "cow" => {
            // Cow. Around 5000 polygons. Octree helps considerably.
            let camera = my_scene::get_cow_camera(image_width, image_height, fov);
            let scene = my_scene::get_cow_scene();
            Some((camera, scene))
        },
        "lucy" => {
            // Lucy. Around 525814+1 primitives. Octree pretty much required. The model is included
            // separately, in another repository. Has skybox.
            let camera = my_scene::get_lucy_camera(image_width, image_height, fov);
            let scene = my_scene::get_lucy_scene();
            Some((camera, scene))
        },
        "sponza" => {
            // Sponza. Around 28K triangles, but more complex than Lucy. 2 lights.
            let camera = my_scene::get_sponza_camera(image_width, image_height, fov);
            let scene = my_scene::get_sponza_scene();
            Some((camera, scene))
        },
        "sibenik" => {
            // Sibenik, around 70K triangles, no texture work, 3 lights.
            let camera = match config.animating {
                true => my_scene::get_sibenik_animation_camera(image_width, image_height, fov),
                false => my_scene::get_sibenik_camera(image_width, image_height, fov)
            };
            let scene = my_scene::get_sibenik_scene();
            Some((camera, scene))
        },
        "heptoroid-white" => {
            // Heptoroid, 114688 tris, 57302 verts
            let camera = my_scene::get_heptoroid_camera(image_width, image_height, fov);
            let scene = my_scene::get_heptoroid_scene();
            Some((camera, scene))
        },
        "heptoroid-shiny" => {
            // Shiny heptoroid, 114688 tris, 57302 verts
            // You can forget about refractions, it's too complex a scene
            let camera = my_scene::get_heptoroid_camera(image_width, image_height, fov);
            let scene = my_scene::get_heptoroid_shiny_scene();
            Some((camera, scene))
        },
        "tachikoma" => {
            // Shiny heptoroid, 114688 tris, 57302 verts
            // You can forget about refractions, it's too complex a scene
            let camera = my_scene::get_tachikoma_camera(image_width, image_height, fov);
            let scene = my_scene::get_tachikoma_scene();
            Some((camera, scene))
        },
        "sphere" => {
            // Sphere skybox test scene
            let camera = match config.animating {
                true => my_scene::get_sphere_animation_camera(image_width, image_height, fov),
                false => my_scene::get_sphere_camera(image_width, image_height, fov)
            };
            let scene = my_scene::get_sphere_scene();
            Some((camera, scene))
        },
        "fresnel" => {
            // Fresnel test scene
            let camera = match config.animating {
                true => my_scene::get_fresnel_animation_camera(image_width, image_height, fov),
                false => my_scene::get_fresnel_camera(image_width, image_height, fov)
            };
            let scene = my_scene::get_fresnel_scene();
            Some((camera, scene))
        },
        _ => None
    };
}

fn main() {
    let start_time = ::time::get_time().sec;

    let program_args = match parse_args(os::args()) {
        Ok(program_args) => program_args,
        Err(error_str) => {
            let mut stderr = io::stderr();
            assert!(stderr.write(error_str.append("\n").as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };
    let config_path = Path::new(program_args.config_file);
    let mut file_handle = match File::open(&config_path) {
        Ok(file) => file,
        Err(err) => {
            let mut stderr = io::stderr();
            assert!(stderr.write(format!("{}", err).append("\n").as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };
    let json_data = match file_handle.read_to_string() {
        Ok(data) => data,
        Err(err) => {
            let mut stderr = io::stderr();
            assert!(stderr.write(format!("{}", err).append("\n").as_bytes()).is_ok());
            os::set_exit_status(1);
            return
        }
    };

    let config: SceneConfig = match json::decode(json_data.as_slice()) {
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
        let animator = raytracer::animator::Animator {
            fps: 25.0,
            length: 5.0,
            renderer: renderer
        };

        println!("Animating - tasks: {}, FPS: {}, length: {}s",
                 renderer.tasks, animator.fps, animator.length);
        animator.animate(camera, shared_scene, config.output_file.as_slice());
        let render_time = ::time::get_time().sec;
        println!("Render done at {} ({}s)",
                 render_time, render_time - scene_time);
    } else {
        // Still frame
        println!("Rendering with {} tasks...", renderer.tasks);
        let image_data = renderer.render(camera, shared_scene);
        let render_time = ::time::get_time().sec;
        println!("Render done at {} ({}s)...\nWriting file...",
                 render_time, render_time - scene_time);

        let out_file = format!("{}{}", config.output_file.as_slice(), ".ppm");
        util::export::to_ppm(image_data, out_file.as_slice());
        let export_time = ::time::get_time().sec;

        println!("Write done: {} ({}s). Written to {}\nTotal: {}s",
                 export_time, export_time - render_time,
                 config.output_file.as_slice(), export_time - start_time);
    }
}
