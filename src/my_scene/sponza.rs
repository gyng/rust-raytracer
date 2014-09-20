#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::Octree;
use raytracer::animator::CameraKeyframe;
use scene::{Camera, Scene};
use vec3::Vec3;

// ~28000 triangles, complex scene with 2 lights
pub fn get_camera(image_width: int, image_height: int, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: 800.0, y: 30.0, z: 90.0 },
        Vec3 { x: -500.0, y: 1000.0, z: -100.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(box SphereLight { position: Vec3 { x: 0.0, y: 3000.0, z: 1000.0 }, color: Vec3 { x: 1.0, y: 0.8, z: 0.4 }, radius: 50.0 });
    lights.push(box SphereLight { position: Vec3 { x: 300.0, y: 300.0, z: 60.0 }, color: Vec3 { x: 0.38, y: 0.32, z: 0.28 }, radius: 20.0 });

    let checker: Box<Texture+Send+Sync> = box CheckerTexture { color1: Vec3::one(), color2: Vec3 { x: 0.15, y: 0.11, z: 0.1 }, scale: 32.0 };

    let stone     = CookTorranceMaterial { k_a: 0.1,  k_d: 0.8, k_s: 0.2, k_sg: 0.2,  k_tg: 0.0, gauss_constant: 50.0, roughness: 1.0, ior: 1.5, ambient: Vec3 { x: 0.88, y: 0.83, z: 0.77 }, diffuse: Vec3 { x: 0.88, y: 0.83, z: 0.77 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let ground    = CookTorranceMaterial { k_a: 0.03, k_d: 0.9, k_s: 0.3, k_sg: 0.5,  k_tg: 0.0, gauss_constant: 25.0, roughness: 0.1, ior: 0.5, ambient: Vec3::one(), diffuse: Vec3 { x: 0.38, y: 0.38, z: 0.5 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: Some(checker.clone()) };
    let cloth     = CookTorranceMaterial { k_a: 0.03, k_d: 0.8, k_s: 0.1, k_sg: 0.05, k_tg: 0.0, gauss_constant: 40.0, roughness: 0.8, ior: 1.3, ambient: Vec3::one(), diffuse: Vec3 { x: 0.85, y: 0.05, z: 0.05 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let shrubbery = CookTorranceMaterial { k_a: 0.03, k_d: 0.8, k_s: 0.2, k_sg: 0.05, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.2, ior: 1.2, ambient: Vec3::one(), diffuse: Vec3 { x: 0.16, y: 0.47, z: 0.11 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    prims.push(box Plane { a: 0.0, b: 1.0, c: 0.0, d: 0.0, material: box ground });

    let sponza_other = ::util::import::from_obj(stone, false, "./docs/assets/models/sponza_other.obj");
    for triangle in sponza_other.triangles.into_iter() { prims.push(triangle); }

    let sponza_column_shrubbery = ::util::import::from_obj(shrubbery, false, "./docs/assets/models/sponza_column_shrubbery.obj");
    for triangle in sponza_column_shrubbery.triangles.into_iter() { prims.push(triangle); }

    let sponza_cloth = ::util::import::from_obj(cloth, false, "./docs/assets/models/sponza_cloth.obj");
    for triangle in sponza_cloth.triangles.into_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        background: Vec3 { x: 0.84, y: 0.34, z: 0.0 },
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
