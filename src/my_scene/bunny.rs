#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::animator::CameraKeyframe;
use scene::{Camera, Scene};
use vec3::Vec3;

// 300 polys, octree is slightly slower than no octree
pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
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
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(Box::new(SphereLight { position: Vec3 { x: 200.0, y: -200.0, z: 100.0 }, color: Vec3::one(), radius: 40.0 }));
    lights.push(Box::new(SphereLight { position: Vec3 { x: -95.0, y: 20.0, z: 170.0 }, color: Vec3 { x: 0.5, y: 0.5, z: 0.3 }, radius: 15.0 }));

    let red   = CookTorranceMaterial { k_a: 0.1, k_d: 0.4, k_s: 0.5, k_sg: 0.5, k_tg: 0.0, gauss_constant: 5.0,  roughness: 0.05, glossiness: 0.0, ior: 0.98, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 0.25, z: 0.1 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let green = CookTorranceMaterial { k_a: 0.0, k_d: 0.4, k_s: 0.6, k_sg: 0.7, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.3,  glossiness: 0.0, ior: 1.5,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.2, y: 0.7, z: 0.2 },  specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let shiny = CookTorranceMaterial { k_a: 0.0, k_d: 0.2, k_s: 0.7, k_sg: 1.0, k_tg: 0.0, gauss_constant: 25.0, roughness: 0.01, glossiness: 0.0, ior: 0.2,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.9, y: 0.9, z: 0.1 },  specular: Vec3 {x: 0.9, y: 0.9, z: 0.1}, transmission: Vec3::zero(), diffuse_texture: None};

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    prims.push(Box::new(Plane { a: 0.0, b: 0.0, c: 1.0, d: -10.0, material: Box::new(green)}));
    prims.push(Box::new(Sphere { center: Vec3 { x: -75.0, y: 60.0, z: 50.0 }, radius: 40.0, material: Box::new(shiny.clone()) }));
    prims.push(Box::new(Sphere { center: Vec3 { x: -75.0, y: 60.0, z: 140.0 }, radius: 40.0, material: Box::new(shiny.clone()) }));
    let bunny = ::util::import::from_obj(red, false, "./docs/assets/models/bunny.obj").ok().expect("failed to load obj model");
    for triangle in bunny.triangles.into_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = prims.into_iter().collect();
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        background: Vec3 { x: 0.3, y: 0.5, z: 0.8 },
        skybox: Some(CubeMap::load(
            "./docs/assets/textures/skyboxes/storm_y_up/left.png",
            "./docs/assets/textures/skyboxes/storm_y_up/right.png",
            "./docs/assets/textures/skyboxes/storm_y_up/down.png",
            "./docs/assets/textures/skyboxes/storm_y_up/up.png",
            "./docs/assets/textures/skyboxes/storm_y_up/front.png",
            "./docs/assets/textures/skyboxes/storm_y_up/back.png"
        ))
    }
}
