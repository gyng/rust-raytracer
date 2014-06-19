extern crate time;

mod geometry;
mod light;
mod material;
mod my_scene;
mod raytracer;
mod scene;
mod util;
mod vec3;

fn main() {
    let start_time = ::time::get_time().sec;
    let image_width = 512;
    let image_height = 512;
    let out_file = "test.ppm";

    println!("Job started at {}...\nLoading scene...", start_time);
    // Cameras, scenes created in ./my_scene.rs
    // Scenes with an octree supplied (see my_scene.rs) will use it.
    // Lower the render quality (especially shadow_samples) for complex scenes.

    // Simplest scene with 9 primitives, no octree
    let camera = my_scene::get_camera(image_width, image_height);
    let scene = my_scene::get_scene();

    // Around 300 primitives, 2 lights. No octree.
    // let camera = my_scene::get_bunny_camera(image_width, image_height);
    // let scene = my_scene::get_bunny_scene();

    // Around 2500 polygons. Octree helps a bit.
    // let camera = my_scene::get_teapot_camera(image_width, image_height);
    // let scene = my_scene::get_teapot_scene();

    // Around 5000 polygons. Octree helps considerably.
    // let camera = my_scene::get_cow_camera(image_width, image_height);
    // let scene = my_scene::get_cow_scene();

    // Around 525814+1 primitives. Octree pretty much required. The model is included
    // separately, in another repository.
    // let camera = my_scene::get_lucy_camera(image_width, image_height);
    // let scene = my_scene::get_lucy_scene();

    let scene_time = ::time::get_time().sec;
    println!("Scene loaded at {} ({}s)...\nRendering...", scene_time, scene_time - start_time);

    let renderer = raytracer::Renderer {
        reflect_depth: 4,
        refract_depth: 6,
        shadow_samples: 64,
        pixel_samples: 2,   // 2 * 2 = 4 samples per pixel
        tasks: 2            // Number of tasks to spawn. Will use up max available threads.
    };
    let image_data = renderer.render(camera, scene);
    let render_time = ::time::get_time().sec;
    println!("Render done at {} ({}s)...\nWriting file...",
             render_time, render_time - scene_time);

    util::export::to_ppm(image_data, out_file);
    let export_time = ::time::get_time().sec;

    println!("Write done: {} ({}s). Written to {}\nTotal: {}s",
        export_time,
        export_time - render_time,
        out_file,
        export_time - start_time
    );
}
