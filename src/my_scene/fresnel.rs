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

// Fresnel test scene
pub fn get_camera(image_width: int, image_height: int, fov: f64) -> Camera {
    let height = 50.0;

    Camera::new(
        Vec3 { x: 50.0, y: height, z: 250.0 },
        Vec3 { x: 50.0, y: 50.0, z: 50.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_animation_camera(image_width: int, image_height: int, fov: f64) -> Camera {
    // State at time t=0
    // A keyframe at time t=0 is automatically created when insert_keyframes is called
    let camera = Camera::new_with_keyframes(
        Vec3 { x: 0.0, y: 1.0, z: 250.0 },
        Vec3 { x: 0.0, y: 1.0, z: 50.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height,
        vec![
            CameraKeyframe {
                time: 2.5,
                position: Vec3 { x: 50.0, y: 100.0, z: 250.0 },
                look_at: Vec3 { x: 0.0, y: 1.0, z: 50.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
            },
            CameraKeyframe {
                time: 5.0,
                position: Vec3 { x: 0.0, y: 200.0, z: 250.0 },
                look_at: Vec3 { x: 0.0, y: 1.0, z: 50.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
            },
            CameraKeyframe {
                time: 7.5,
                position: Vec3 { x: -50.0, y: 100.0, z: 250.0 },
                look_at: Vec3 { x: 0.0, y: 1.0, z: 50.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
            },
            CameraKeyframe {
                time: 10.0,
                position: Vec3 { x: 0.0, y: 1.0, z: 250.0 },
                look_at: Vec3 { x: 0.0, y: 1.0, z: 50.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
            },
        ]
    );

    camera
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight { position: Vec3 { x: 50.0, y: 80.0, z: 50.0 }, color: Vec3::one(), radius: 10.0 });

    let checker: Box<Texture+Send+Share> = box CheckerTexture { color1: Vec3::one(), color2: Vec3 { x: 0.1, y: 0.1, z: 0.1 }, scale: 32.0 };
    let checker_red          = CookTorranceMaterial { k_a: 0.0, k_d: 1.0, k_s: 0.0, k_sg: 0.0, k_tg: 0.0, gauss_constant: 1.0, roughness: 0.15, ior: 1.5,  ambient: Vec3::one(), diffuse: Vec3 { x: 0.6, y: 0.6, z: 0.6 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: Some(checker.clone()) };
    let shiny                = CookTorranceMaterial { k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, ior: 0.15, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3::zero(), diffuse_texture: None };
    let global_specular_only = CookTorranceMaterial { k_a: 0.0, k_d: 0.0, k_s: 0.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, ior: 1.5,  ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3::zero(), diffuse_texture: None };
    let refract              = CookTorranceMaterial { k_a: 0.0, k_d: 0.0, k_s: 1.0, k_sg: 1.0, k_tg: 1.0, gauss_constant: 5.0, roughness: 0.01, ior: 3.0,  ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3::zero(), diffuse_texture: None };

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane { a: 0.0,  b:  0.0, c: 1.0, d: 0.0, material: box checker_red.clone() }); // Ahead
    prims.push(box Plane { a: 0.0,  b:  1.0, c: 0.0, d: 0.0, material: box global_specular_only.clone() }); // Bottom
    prims.push(box Sphere { center: Vec3 {x: 30.0, y: 15.0, z: 20.0 }, radius: 15.0, material: box shiny.clone() });
    prims.push(box Sphere { center: Vec3 {x: 70.0, y: 17.0, z: 60.0 }, radius: 17.0, material: box refract.clone() });

    Scene {
        lights: lights,
        prim_strat: box VecPrimContainer::new(prims),
        background: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
        skybox: None
    }
}
