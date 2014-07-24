rust-raytracer
==============

![ScreenShot](https://raw.githubusercontent.com/gyng/rust-raytracer/master/docs/sample_render.png)

Early-stage raytracer in Rust. Developed on Rust `0.12.0-pre-nightly`.

## Usage

0. `git clone --recursive https://github.com/gyng/rust-raytracer.git`. This clones the smaller models into the project directory as well.
1. Scenes are created in `my_scene.rs`
2. Load the scene/camera in main.rs, and send it to the renderer (already done for default scene)
3. Compile: `rustc main.rs`
4. Run compiled program

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
* Basic textures (checker, uv, simple image)
* Skybox (cubemap)

## Missing/potential features

* Proper Fresnel reflectance
* Simple animation
* Scene description
