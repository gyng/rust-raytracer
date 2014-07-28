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

    // Box. Simplest scene with 9 primitives, no octree
    let camera = my_scene::get_camera(image_width, image_height);
    let scene = my_scene::get_scene();

    // Bunny. Around 300 primitives, 2 lights. No octree. Has skybox, textures are
    // in another repository.
    // let camera = my_scene::get_bunny_camera(image_width, image_height);
    // let scene = my_scene::get_bunny_scene();

    // Teapot. Around 2500 polygons. Octree helps a bit. Has skybox.
    // let camera = my_scene::get_teapot_camera(image_width, image_height);
    // let scene = my_scene::get_teapot_scene();

    // Cow. Around 5000 polygons. Octree helps considerably.
    // let camera = my_scene::get_cow_camera(image_width, image_height);
    // let scene = my_scene::get_cow_scene();

    // Lucy. Around 525814+1 primitives. Octree pretty much required. The model is included
    // separately, in another repository. Has skybox.
    // let camera = my_scene::get_lucy_camera(image_width, image_height);
    // let scene = my_scene::get_lucy_scene();

    // Sponza. Around 28K triangles, but more complex than Lucy. 2 lights.
    // let camera = my_scene::get_sponza_camera(image_width, image_height);
    // let scene = my_scene::get_sponza_scene();

    // Sibenik, around 70K triangles, no texture work, 3 lights.
    // let camera = my_scene::get_sibenik_camera(image_width, image_height);
    // let scene = my_scene::get_sibenik_scene();

    // Sphere skybox test scene
    // let camera = my_scene::get_sphere_camera(image_width, image_height);
    // let scene = my_scene::get_sphere_scene();

    // Fresnel test scene
    // let camera = my_scene::get_fresnel_camera(image_width, image_height);
    // let scene = my_scene::get_fresnel_scene();

    let scene_time = ::time::get_time().sec;
    println!("Scene loaded at {} ({}s)...", scene_time, scene_time - start_time);

    let renderer = raytracer::Renderer {
        reflect_depth: 4,          // 1 = no reflection, only surface shading. (4)
        refract_depth: 6,          // Going in and out of one object takes two refracts. (6)
        shadow_samples: 64,        // For soft shadows. 0 for no shadows. (64)
        pixel_samples: 2,          // 2 * 2 = 4 samples per pixel (2)
        tasks: std::os::num_cpus() // Number of tasks to spawn. Will use up max available cores.
    };

    println!("Rendering with {} tasks...", renderer.tasks);
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
             export_time - start_time);
}
