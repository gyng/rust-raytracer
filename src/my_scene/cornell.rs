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
use raytracer::compositor::ColorRGBA;
use scene::{Camera, Scene};
use vec3::Vec3;

// 10 primitives, octree is super inefficient for this scene
pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: 50.0, y: 25.0, z: 150.0 },
        Vec3 { x: 50.0, y: 50.0, z: 50.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(Box::new(SphereLight {position: Vec3 { x: 50.0, y: 80.0, z: 50.0 }, color: Vec3::one(), radius: 10.0 }));

    // Example of a textured material
    let checker: Box<Texture+Send+Sync> = Box::new(CheckerTexture { color1: ColorRGBA::white(), color2: ColorRGBA::new_rgb(0.8, 0.1, 0.1), scale: 16.0 });
    let checker_grey = CookTorranceMaterial { k_a: 0.0, k_d: 1.0, k_s: 0.0, k_sg: 0.0, k_tg: 0.0, gauss_constant: 1.0,  roughness: 0.15, glossiness: 0.0, ior: 0.7,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.6, y: 0.6, z: 0.6 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: Some(checker.clone()) };

    // Example of a short-form material definition using defaults
    // let grey      = CookTorranceMaterial { k_a: 0.0, k_d: 1.0, k_s: 1.0, k_sg: 0.0, k_tg: 0.0, gauss_constant: 1.0,  roughness: 0.15, glossiness: 0.0, ior: 1.5,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.6, y: 0.6, z: 0.6 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let grey         = CookTorranceMaterial { diffuse: Vec3 { x: 0.6, y: 0.6, z: 0.6 }, ..Default::default() };

    let blue         = CookTorranceMaterial { k_a: 0.0, k_d: 0.3, k_s: 0.7, k_sg: 0.0, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.1,  glossiness: 0.0, ior: 1.3,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.1, y: 0.1, z: 1.0 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let red          = PhongMaterial        { k_a: 0.0, k_d: 0.6, k_s: 0.4, k_sg: 1.0, k_tg: 0.0, shininess: 10.0,                       glossiness: 0.0, ior: 0.5,  ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 0.0, z: 0.0 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let green        = PhongMaterial        { k_a: 0.0, k_d: 0.9, k_s: 0.1, k_sg: 0.5, k_tg: 0.0, shininess: 10.0,                       glossiness: 0.0, ior: 0.7,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.0, y: 1.0, z: 0.0 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let shiny        = CookTorranceMaterial { k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01,  glossiness: 0.0, ior: 0.25, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3::zero(), diffuse_texture: None };
    let shiny_glossy = CookTorranceMaterial { k_a: 0.0, k_d: 0.7, k_s: 1.0, k_sg: 0.4, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01,  glossiness: 0.2, ior: 0.25, ambient: Vec3::one(), diffuse: Vec3 { x: 0.3, y: 0.3, z: 1.0 }, specular: Vec3 { x: 0.3, y: 0.3, z: 1.0 }, transmission: Vec3::zero(), diffuse_texture: None };
    let refract      = CookTorranceMaterial { k_a: 0.0, k_d: 0.0, k_s: 1.0, k_sg: 1.0, k_tg: 1.0, gauss_constant: 5.0, roughness: 0.01,  glossiness: 0.0, ior: 3.0,  ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3 { x: 0.8, y: 0.8, z: 0.8 }, diffuse_texture: None };

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    prims.push(Box::new(Plane { a:  0.0, b:  0.0, c: 1.0, d: 0.0,   material: Box::new(grey.clone()) }));         // Ahead
    prims.push(Box::new(Plane { a:  0.0, b:  1.0, c: 0.0, d: 0.0,   material: Box::new(checker_grey.clone()) })); // Bottom
    prims.push(Box::new(Plane { a:  0.0, b: -1.0, c: 0.0, d: 100.0, material: Box::new(grey.clone()) }));         // Top
    prims.push(Box::new(Plane { a:  1.0, b:  0.0, c: 0.0, d: 0.0,   material: Box::new(red.clone()) }));          // Left
    prims.push(Box::new(Plane { a: -1.0, b:  0.0, c: 0.0, d: 100.0, material: Box::new(green.clone()) }));        // Right
    prims.push(Box::new(Sphere { center: Vec3 { x: 30.0, y: 15.0, z: 20.0 }, radius: 15.0, material: Box::new(shiny.clone())}));
    prims.push(Box::new(Sphere { center: Vec3 { x: 70.0, y: 17.0, z: 60.0 }, radius: 17.0, material: Box::new(refract.clone())}));
    prims.push(Box::new(Sphere { center: Vec3 { x: 50.0, y: 50.0, z: 20.0 }, radius: 10.0, material: Box::new(shiny_glossy.clone())}));
    prims.push(Box::new(Sphere { center: Vec3 { x: 20.0, y: 13.0, z: 90.0 }, radius: 13.0, material: Box::new(blue.clone())}));
    prims.push(Box::new(Triangle::auto_normal(Vec3 { x: 20.0, y: 95.0, z: 20.0 }, Vec3 { x: 15.0, y: 50.0, z: 40.0 }, Vec3 { x: 35.0, y: 50.0, z: 35.0 }, (0.5, 1.0), (0.0, 0.0), (1.0, 0.0), Box::new(blue))));

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        background: Vec3::one(),
        skybox: None
    }
}
