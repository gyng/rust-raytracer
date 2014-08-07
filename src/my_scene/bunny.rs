#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::{Octree, VecPrimContainer};
use raytracer::animator::CameraKeyframe;
use scene::{Camera, Scene};
use vec3::Vec3;

// 300 polys, octree is slightly slower than no octree
pub fn get_camera(image_width: int, image_height: int, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: 0.0, y: -150.0, z: 30.0 },
        Vec3 { x: 0.0, y: 60.0, z: 50.0 },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight { position: Vec3 { x: 200.0, y: -200.0, z: 100.0 }, color: Vec3::one(), radius: 40.0 });
    lights.push(box SphereLight { position: Vec3 { x: -95.0, y: 20.0, z: 170.0 }, color: Vec3 { x: 0.5, y: 0.5, z: 0.3 }, radius: 15.0 });

    let red   = CookTorranceMaterial { k_a: 0.1, k_d: 0.4, k_s: 0.5, k_sg: 0.5, k_tg: 0.0, gauss_constant: 5.0,  roughness: 0.05, ior: 0.98, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 0.25, z: 0.1 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let green = CookTorranceMaterial { k_a: 0.0, k_d: 0.4, k_s: 0.6, k_sg: 0.7, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.3,  ior: 1.5,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.2, y: 0.7, z: 0.2 },  specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let shiny = CookTorranceMaterial { k_a: 0.0, k_d: 0.2, k_s: 0.7, k_sg: 1.0, k_tg: 0.0, gauss_constant: 25.0, roughness: 0.01, ior: 0.2,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.9, y: 0.9, z: 0.1 },  specular: Vec3 {x: 0.9, y: 0.9, z: 0.1}, transmission: Vec3::zero(), diffuse_texture: None};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane { a: 0.0, b: 0.0, c: 1.0, d: -10.0, material: box green});
    prims.push(box Sphere { center: Vec3 { x: -75.0, y: 60.0, z: 50.0 }, radius: 40.0, material: box shiny.clone() });
    prims.push(box Sphere { center: Vec3 { x: -75.0, y: 60.0, z: 140.0 }, radius: 40.0, material: box shiny.clone() });
    let bunny = ::util::import::from_obj(Vec3::zero(), 1.0, red, false, "./docs/assets/models/bunny.obj");
    for triangle in bunny.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prim_strat: box octree,
        background: Vec3 { x: 0.3, y: 0.5, z: 0.8 },
        skybox: Some(CubeMap::load(
            "./docs/assets/textures/skyboxes/storm_y_up/left.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/right.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/down.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/up.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/front.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/back.ppm"
        ))
    }
}
