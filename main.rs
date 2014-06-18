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
    let image_width = 600;
    let image_height = 600;
    let out_file = "test.ppm";

    println!("Render started at {}...", start_time);
    // Camera, scene created in ./my_scene.rs
    let camera = my_scene::get_camera(image_width, image_height);
    let scene = my_scene::get_scene();

    // let camera = my_scene::get_bunny_camera(image_width, image_height);
    // let scene = my_scene::get_bunny_scene();

    let renderer = raytracer::Renderer {
        reflect_depth: 4,
        refract_depth: 6,
        use_octree: false,  // Unimplemented
        shadow_samples: 64,
        pixel_samples: 2,   // 2 * 2 = 4 samples per pixel
        tasks: 8            // Minimum number of tasks to spawn. Will use up max available threads.
    };
    let image_data = renderer.render(camera, scene);
    let render_time = ::time::get_time().sec;
    println!("Render done at {} ({}s)...\nWriting file...", render_time, render_time - start_time);

    util::export::to_ppm(image_data, out_file);
    let export_time = ::time::get_time().sec;

    println!("Write done: {} ({}s). Written to {}\nTotal: {}s",
        export_time,
        export_time - render_time,
        out_file,
        export_time - start_time
    );
}
