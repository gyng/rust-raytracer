rust-raytracer
==============

![ScreenShot](https://raw.githubusercontent.com/gyng/rust-raytracer/master/docs/sample_render.png)

Early-stage raytracer in Rust. Developed on Rust `0.11.0-pre-nightly`.

## Usage

1. Compile: `rustc main.rs`
2. Scene is created in `my_scene.rs`
3. Run compiled program

## Features

* Reflections
* Refractions
* Multi-threading
* Soft shadows
* Supersampling
* Cook-Torrance, Phong materials
* Sphere, plane, triangle primitives
* Point, sphere lights

## Missing/potential features

* Spatial partitioning (octree/k-d tree)
* Textures
* Proper Fresnel reflectance
* OBJ meshes
* Scene description
* Lenses
