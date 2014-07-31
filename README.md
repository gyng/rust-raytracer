rust-raytracer
==============

![ScreenShot](https://raw.githubusercontent.com/gyng/rust-raytracer/master/docs/sample_render.png)

Early-stage raytracer in Rust. Developed on Rust `0.12.0-pre-nightly`.

[Gallery](http://gyng.github.io/rust-raytracer-gallery/) <br>
[Gallery repository](https://github.com/gyng/rust-raytracer-gallery) <br>
[Assets repository](https://github.com/gyng/raytracer-assets)


## Usage

1. `git clone --recursive https://github.com/gyng/rust-raytracer.git`. This clones the smaller models and textures into the project directory as well.
2. Convert PNG textures into PPM by running appropriate scripts (`ruby ./all_to_ppm.rb` in `./docs/assets/textures/skyboxes/`)
3. Compile: `rustc main.rs`
4. Edit `sample-config.json` if you wish to render a scene besides the default,
   or if you wish to tweak the renderer parameters
5. Run compiled program, passing sample-config.json as an argument. e.g.: `./main
   sample-config.json`
6. To update (assets) submodules only: `git submodule foreach git pull`
7. To convert frames into a video `ffmpeg -i test%06d.ppm -b 1500k out.webm`


## Available Scenes
* box
* bunny
* cow
* fresnel
* lucy
* sibenik
* sphere
* sponza
* teapot
* heptoroid-white
* heptoroid-shiny
* tachikoma


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
* Caustics/global illumination (in progress on `photon-trace` branch)
