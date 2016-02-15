#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::animator::CameraKeyframe;
use raytracer::animator::easing::Easing;
use scene::{Camera, Scene};
use vec3::Vec3;

// Skybox test scene
pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    let up = Vec3 { x: 0.0, y: 1.0, z: 0.0 }; // y-up
    Camera::new(
        Vec3 { x: 0.0, y: 0.0, z: 10.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        up,
        fov,
        image_width,
        image_height
    )

    // let up = Vec3 { x: 0.0, y: 0.0, z: 1.0 }; // z-up
    // Camera::new(
    //     Vec3 { x: 0.0, y: 10.0, z: 0.0 },
    //     Vec3 { x: 0.0, y: 0.0, z: 0.0 },
    //     up,
    //     fov,
    //     image_width,
    //     image_height
    // )
}

pub fn get_animation_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    // State at time t=0
    // A keyframe at time t=0 is automatically created when insert_keyframes is called
    let camera = Camera::new_with_keyframes(
        Vec3 { x: 0.0, y: 0.0, z: 10.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height,
        vec![
            CameraKeyframe {
                time: 2.5,
                position: Vec3 { x: 10.0, y: 0.0, z: 0.0 },
                look_at: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                easing: Easing::linear()
            },
            CameraKeyframe {
                time: 5.0,
                position: Vec3 { x: 0.0, y: 0.0, z: -10.0 },
                look_at: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                easing: Easing::linear()
            },
            CameraKeyframe {
                time: 7.5,
                position: Vec3 { x: -10.0, y: 0.0, z: 0.0 },
                look_at: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                easing: Easing::linear()
            },
            CameraKeyframe {
                time: 10.0,
                position: Vec3 { x: 0.0, y: 0.0, z: 10.0 },
                look_at: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                easing: Easing::linear()
            },
        ]
    );

    camera
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(Box::new(SphereLight { position: Vec3 { x: 3.0, y: 10.0, z: 6.0 }, color: Vec3::one(), radius: 5.0 }));

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    let shiny = CookTorranceMaterial { k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, glossiness: 0.0, ior: 0.05, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3::zero(), diffuse_texture: None };
    prims.push(Box::new(Sphere { center: Vec3::zero(), radius: 2.0, material: Box::new(shiny) }));

    println!("Generating octree...");
    let octree = prims.into_iter().collect();
    println!("Octree generated...");

    // For y as up
    Scene {
        lights: lights,
        background: Vec3 { x: 0.3, y: 0.5, z: 0.8 },
        octree: octree,
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


pub struct SphereConfig;

impl super::SceneConfig for SphereConfig {
    fn get_camera(&self, image_width: u32, image_height: u32, fov: f64) -> Camera {
        get_camera(image_width, image_height, fov)
    }

    fn get_animation_camera(&self, image_width: u32, image_height: u32, fov: f64) -> Camera {
        get_animation_camera(image_width, image_height, fov)
    }

    fn get_scene(&self) -> Scene {
        get_scene()
    }
}