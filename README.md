rust-raytracer
==============

![ScreenShot](https://raw.githubusercontent.com/gyng/rust-raytracer/master/docs/sample_render.png)

Early-stage raytracer in Rust. Developed on Rust `0.12.0-pre-nightly`.

## Usage

1. `git clone --recursive https://github.com/gyng/rust-raytracer.git`. This clones the smaller models and textures into the project directory as well.
2. Convert PNG textures into PPM by running appropriate scripts (`ruby ./all_to_ppm.rb` in `./docs/assets/textures/skyboxes/`)
3. Scenes are created in `my_scene.rs`
4. Load the scene/camera in main.rs, and send it to the renderer (already done for default scene)
5. Compile: `rustc main.rs`
6. Run compiled program
7. To update (assets) submodules only: `git submodule foreach git pull`
8. To convert frames into a video `ffmpeg -i test%06d.ppm -b 1500k out.webm`


## Features

* Reflections
* Refractions
* Multi-threading
* Soft shadows
* Supersampling
* Cook-Torrance, Phong materials
* Sphere, plane, triangle primitives
* Point, sphere lights
* Very limited OBJ model and mesh support
* Basic spatial partitioning (octree)
* Basic textures (checker, uv, image)
* Skybox (cubemap)
* Basic camera animation

## Missing/potential features

* Scene description
