rust-raytracer
==============
[![Build Status](https://travis-ci.org/gyng/rust-raytracer.svg?branch=master)](https://travis-ci.org/gyng/rust-raytracer)

![ScreenShot](https://raw.githubusercontent.com/gyng/rust-raytracer/master/docs/sample_render.png)

A raytracer in Rust. [Does not compile on Rust stable, only on nightly](https://github.com/gyng/rust-raytracer/issues/34#issuecomment-112358227).

[Gallery](http://gyng.github.io/rust-raytracer-gallery/) <br>
[Gallery repository](https://github.com/gyng/rust-raytracer-gallery) <br>
[Assets repository](https://github.com/gyng/raytracer-assets)


## Usage

1. Clone the project. `--recursive` clones most sample models and textures into the project directory as well.

        git clone --recursive https://github.com/gyng/rust-raytracer.git

2. Convert PNG textures into PPM by running appropriate scripts in `./docs/assets/textures/skyboxes/`.
   This assumes you have, at the minimum, ImageMagick installed.

        cd ./docs/assets/textures/skyboxes/

        with Ruby and ImageMagick:
        ruby ./all_to_ppm.rb

        with plain ImageMagick:
        find . -name '*.png' -execdir mogrify -format ppm {} \;

3. Compile

        cargo build --release

4. Edit `sample-config.json` if you wish to render a scene besides the default,
   or if you wish to tweak the renderer parameters

5. Run the compiled program, passing the render configuration as an argument.
   If rendering a provided scene, run the binary in the project root so it can find the models and textures.

        ./main sample-config.json

6. Alternatively

        cargo run --release sample-config.json


### Useful commands

* To update (assets) submodules only: `git submodule foreach git pull`
* To convert frames into a video `ffmpeg -i test%06d.ppm -b 2000k out.webm`
* Scenes are created in `./myscene/`. To hook up a scene, add it to `./myscene/mod.rs` and `get_camera_and_scene(&SceneConfig)` in `main.rs`.


## Available Scenes

These should use 30deg fov for squares and 45deg fov for 16:9.

* box
* bunny
* cow
* fresnel (0s-10s animation)
* lucy
* sibenik (0s-7s animation)
* sphere (0s-10s animation)
* sponza (45deg fov for a square; 67.5deg for 16:9)
* teapot
* heptoroid-white
* heptoroid-shiny
* heptoroid-refractive
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
* Limited OBJ model and mesh support
* Mesh transformations (4x4 matrices)
* Basic spatial partitioning (octree)
* Basic textures (checker, uv, image)
* Skybox (cubemap)
* Basic camera animation


## Missing/potential features

* Scene description
* Caustics/global illumination (in progress on `photon-trace` branch)
