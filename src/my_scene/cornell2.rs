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

// 10 primitives, octree is super inefficient for this scene
pub fn get_camera(image_width: int, image_height: int, fov: f64) -> Camera {
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
    lights.push(box PointLight {position: Vec3 { x: 50.0, y: 80.0, z: 50.0 }, color: Vec3 { x: 1.0, y: 1.0, z: 0.9 } });

    // let green   = PhongMaterial        { k_a: 0.0, k_d: 0.9, k_s: 0.1, k_sg: 0.5, k_tg: 0.0, shininess: 10.0,                       ior: 0.7,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.0, y: 1.0, z: 0.0 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    // let green        = PhongMaterial        { k_a: 0.0, k_d: 0.9, k_s: 0.1, k_dg: 0.4, k_sg: 0.5, k_tg: 0.0, shininess: 10.0,                       ior: 0.7,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.0, y: 1.0, z: 0.0 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let green   = PhongMaterial        { k_a: 0.0, k_d: 0.9, k_s: 0.1, k_dg: 0.9, k_sg: 0.0, k_tg: 0.0, shininess: 10.0,                       ior: 0.7,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.0, y: 1.0, z: 0.0 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let red     = PhongMaterial        { k_a: 0.0, k_d: 0.9, k_s: 0.1, k_dg: 0.9, k_sg: 0.0, k_tg: 0.0, shininess: 10.0,                       ior: 0.7,  ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 0.0, z: 0.0 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let white   = PhongMaterial        { k_a: 0.0, k_d: 1.0, k_s: 0.0, k_dg: 0.0, k_sg: 0.0, k_tg: 0.0, shininess: 10.0,                       ior: 0.7,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.8, y: 0.8, z: 0.8 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let shiny   = CookTorranceMaterial { k_a: 0.0, k_d: 0.2, k_s: 1.0, k_dg: 0.1, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01,  ior: 0.25, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3::zero(), diffuse_texture: None };
    let refract = CookTorranceMaterial { k_a: 0.0, k_d: 0.0, k_s: 0.0, k_dg: 0.0, k_sg: 0.0, k_tg: 1.0, gauss_constant: 5.0, roughness: 0.01,  ior: 3.0,  ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3 { x: 0.95, y: 0.95, z: 0.95 }, diffuse_texture: None };

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    prims.push(box Plane { a:  0.0, b:  0.0, c: 1.0, d: 0.0,   material: box white.clone() }); // Ahead
    prims.push(box Plane { a:  0.0, b:  1.0, c: 0.0, d: 0.0,   material: box white.clone() }); // Bottom
    prims.push(box Plane { a:  0.0, b: -1.0, c: 0.0, d: 100.0, material: box white.clone() }); // Top
    prims.push(box Plane { a:  1.0, b:  0.0, c: 0.0, d: 0.0,   material: box red.clone() });   // Left
    prims.push(box Plane { a: -1.0, b:  0.0, c: 0.0, d: 100.0, material: box green.clone() }); // Right
    prims.push(box Sphere { center: Vec3 { x: 65.0, y: 15.0, z: 55.0 }, radius: 15.0, material: box refract.clone() });
    prims.push(box Sphere { center: Vec3 { x: 15.0, y: 20.0, z: 25.0 }, radius: 20.0, material: box shiny.clone() });
    prims.push(box Sphere { center: Vec3 { x: 15.0, y: 60.0, z: 15.0 }, radius: 10.0, material: box white.clone() });
    prims.push(box Sphere { center: Vec3 { x: 85.0, y: 60.0, z: 15.0 }, radius: 10.0, material: box white.clone() });

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
