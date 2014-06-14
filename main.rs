extern crate time;

use vec3::Vec3;
use pointlight::PointLight;
use sphere::Sphere;
use plane::Plane;
use phongmaterial::PhongMaterial;
use light::Light;
use prim::Prim;

mod vec3;
mod ray;
mod camera;
mod prim;
mod sphere;
mod plane;
mod light;
mod pointlight;
mod material;
mod flatmaterial;
mod phongmaterial;
mod intersection;
mod scene;
mod renderer;
mod export;

fn main() {
    let start_time = ::time::get_time().sec;

    let image_width = 480;
    let image_height = 360;
    let out_file = "test.ppm";

    let max_lights = 10;
    let max_prims = 1000;

    let mut lights: Vec<Box<Light>> = Vec::with_capacity(max_lights);
    lights.push(box PointLight {position: Vec3 {x: 50.0, y: 90.0, z: 50.0}, color: Vec3::one()});

    let grey    = PhongMaterial {k_a: 0.0, k_d: 1.0, k_s: 0.0, k_sg: 0.0, k_tg: 0.0, shininess: 10.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 0.6, y: 0.6, z: 0.6}, specular: Vec3::one(), transmission: Vec3::zero()};
    let red     = PhongMaterial {k_a: 0.0, k_d: 0.6, k_s: 0.4, k_sg: 0.3, k_tg: 0.0, shininess: 10.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 0.0, z: 0.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let green   = PhongMaterial {k_a: 0.0, k_d: 0.9, k_s: 0.1, k_sg: 0.1, k_tg: 0.0, shininess: 10.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 0.0, y: 1.0, z: 0.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let blue    = PhongMaterial {k_a: 0.0, k_d: 0.4, k_s: 0.6, k_sg: 0.0, k_tg: 0.0, shininess: 10.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 0.0, y: 0.0, z: 1.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let shiny   = PhongMaterial {k_a: 0.0, k_d: 0.5, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, shininess: 50.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let refract = PhongMaterial {k_a: 0.0, k_d: 0.0, k_s: 0.0, k_sg: 0.0, k_tg: 1.0, shininess: 10.0, ior: 3.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3::one(), transmission: Vec3::one()};

    let mut prims: Vec<Box<Prim>> = Vec::with_capacity(max_prims);
    prims.push(box Plane {a: 0.0,  b:  0.0, c: 1.0, d: 0.0,   material: box grey  }); // Ahead
    prims.push(box Plane {a: 0.0,  b: -1.0, c: 0.0, d: 100.0, material: box grey  }); // Bottom
    prims.push(box Plane {a: 0.0,  b:  1.0, c: 0.0, d: 0.0,   material: box grey  }); // Top
    prims.push(box Plane {a: 1.0,  b:  0.0, c: 0.0, d: 0.0,   material: box red   }); // Left
    prims.push(box Plane {a: -1.0, b:  0.0, c: 0.0, d: 100.0, material: box green }); // Right
    prims.push(box Sphere {center: Vec3 {x: 30.0, y: 15.0, z: 20.0}, radius: 15.0, material: box shiny});
    prims.push(box Sphere {center: Vec3 {x: 70.0, y: 17.0, z: 80.0}, radius: 17.0, material: box refract});

    let camera = camera::Camera::new(
        Vec3 {x: 50.0, y: 25.0, z: 150.0},
        Vec3 {x: 50.0, y: 50.0, z: 50.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        45.0,
        image_width,
        image_height
    );

    let scene = scene::Scene {
        lights: lights,
        prims: prims,
        background: Vec3::one()
    };

    let renderer = renderer::Renderer {
        reflect_depth: 4,
        refract_depth: 8,
        use_octree: false,
        shadows: true,
        threads: 1
    };
    let image_data = renderer.render(camera, scene);
    let render_time = ::time::get_time().sec;

    ::export::to_ppm(image_data, image_width, image_height, out_file);
    let export_time = ::time::get_time().sec;

    println!("Start: {}, Render done: {} ({}s), Write done: {} ({}s), Total: {}s, written to {}",
        start_time,
        render_time,
        render_time - start_time,
        export_time,
        export_time - render_time,
        export_time - start_time,
        out_file
    );
}
