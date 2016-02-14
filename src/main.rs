#![deny(unused_imports)]

extern crate image;
extern crate num;
extern crate num_cpus;
extern crate rand;
extern crate rustc_serialize;
extern crate threadpool;
extern crate time;

use std::fs::File;
use std::io::{self, Read, Write};
use std::env;
use std::process;
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
struct SceneConfig {
    name: String,
    size: (u32, u32),
    fov: f64,
    reflect_depth: u32,
    refract_depth: u32,
    shadow_samples: u32,
    gloss_samples: u32,
    pixel_samples: u32,
    output_file: String,
    animating: bool,
    fps: f64,
    time_slice: (f64, f64),
    starting_frame_number: u32
}

fn parse_args(args: env::Args) -> Result<ProgramArgs, String> {
    let args = args.collect::<Vec<String>>();
    if args.len() == 0 {
        panic!("Args do not even include a program name");
    }

    let program_name = &args[0];
    match args.len() {
        // I wouldn't expect this in the wild
        0 => unreachable!(),
        1 => Err(format!("Usage: {} scene_config.json", program_name)),
        2 => Ok(ProgramArgs { config_file: args[1].clone() }),
        _ => Err(format!("Usage: {} scene_config.json", program_name)),
    }
}

fn main() {
    let start_time = ::time::get_time().sec;

    let program_args = match parse_args(env::args()) {
        Ok(program_args) => program_args,
        Err(error_str) => {
            write!(&mut io::stderr(), "{}\n", error_str).unwrap();
            process::exit(1);
        }
    };
    let mut file_handle = match File::open(&program_args.config_file) {
        Ok(file) => file,
        Err(err) => {
            write!(&mut io::stderr(), "{}\n", err).unwrap();
            process::exit(1);
        }
    };

    let mut json_data = String::new();
    if let Err(ref err) = file_handle.read_to_string(&mut json_data) {
        write!(&mut io::stderr(), "{}\n", err).unwrap();
        process::exit(1);
    }

    let config: SceneConfig = match json::decode(&json_data) {
        Ok(data) => data,
        Err(err) => {
            let msg = match err {
                MissingFieldError(field_name) => {
                    format!("parse failure, missing field ``{}''\n", field_name)
                },
                _ => {
                    format!("parse failure: {:?}", err)
                }
            };
            write!(&mut io::stderr(), "{}\n", msg).unwrap();
            process::exit(1);
        }
    };

    println!("Job started at {}...\nLoading scene...", start_time);

    let scene_config = match my_scene::scene_by_name(&config.name) {
        Some(scene_config) => scene_config,
        None => {
            write!(&mut io::stderr(), "unknown scene ``{}''\n", config.name).unwrap();
            process::exit(1);
        }
    };

    let (image_width, image_height) = config.size;
    let fov = config.fov;

    // Hackish solution for animator
    let shared_scene = Arc::new(scene_config.get_scene());

    let camera = if config.animating {
        scene_config.get_animation_camera(image_width, image_height, fov)
    } else {
        scene_config.get_camera(image_width, image_height, fov)
    };

    let scene_time = ::time::get_time().sec;
    println!("Scene loaded at {} ({}s)...", scene_time, scene_time - start_time);

    let render_options = raytracer::RenderOptions {
        reflect_depth: config.reflect_depth,
        refract_depth: config.refract_depth,
        shadow_samples: config.shadow_samples,
        gloss_samples: config.gloss_samples,
        pixel_samples: config.pixel_samples,
    };

    let renderer = raytracer::Renderer {
        options: render_options,
        tasks: ::num_cpus::get(), // Number of tasks to spawn. Will use up max available cores.
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
                 ::num_cpus::get(), animator.fps, animator.animate_from, animator.animate_to,
                 animator.starting_frame_number);
        animator.animate(camera, shared_scene, &config.output_file);
        let render_time = ::time::get_time().sec;
        println!("Render done at {} ({}s)",
                 render_time, render_time - scene_time);
    } else {
        // Still frame
        println!("Rendering with {} tasks...", ::num_cpus::get());
        let image_data = renderer.render(camera, shared_scene);
        let render_time = ::time::get_time().sec;
        println!("Render done at {} ({}s)...\nWriting file...",
                 render_time, render_time - scene_time);

        let out_file = format!("{}{}", config.output_file, ".ppm");
        util::export::to_ppm(image_data, &out_file);
        let export_time = ::time::get_time().sec;

        println!("Write done: {} ({}s). Written to {}\nTotal: {}s",
                 export_time, export_time - render_time,
                 config.output_file, export_time - start_time);
    }
}
