rust-raytracer
==============

![ScreenShot](https://raw.githubusercontent.com/gyng/rust-raytracer/master/sample_render.png)

Early-stage raytracer in Rust. Developed on Rust `0.11.0-pre-nightly`.

### Compilation

`rustc main.rs`

### Features

* Reflections
* Refractions
* Soft shadows
* Supersampling
* Cook-Torrance, Phong materials
* Sphere, plane primitives
* Point, sphere lights

### Missing/potential features

* Threading (can't get it to work)
* Spatial partitioning (octree/k-d tree)
* Textures
* Proper Fresnel reflectance
* Triangles
* OBJ meshes
* Scene description
* Lenses
