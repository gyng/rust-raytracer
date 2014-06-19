rust-raytracer
==============

![ScreenShot](https://raw.githubusercontent.com/gyng/rust-raytracer/master/docs/sample_render.png)

Early-stage raytracer in Rust. Developed on Rust `0.11.0-pre-nightly`.

## Usage

1. Scene is created in `my_scene.rs`
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

## Missing/potential features

* Textures
* Proper Fresnel reflectance
* Simple animation
* Scene description
