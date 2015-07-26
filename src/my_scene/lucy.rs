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

// 50000 polys, model not included!
pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: -1500.0, y: 300.0, z: 600.0 },
        Vec3 { x: 0.0, y: 400.0, z: -200.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(Box::new(SphereLight { position: Vec3 { x: -1400.0, y: 200.0, z: 100.0 }, color: Vec3 { x: 1.0, y: 0.80, z: 0.40 }, radius: 50.0 }));

    let grey = CookTorranceMaterial { k_a: 0.0, k_d: 0.5, k_s: 0.8, k_sg: 0.5, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.1, glossiness: 0.0, ior: 0.4, ambient: Vec3::one(), diffuse: Vec3 { x: 0.6, y: 0.6, z: 0.65 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    let lucy = ::util::import::from_obj(grey, true, "./docs/assets/models/lucy.obj").ok().expect("failed to load obj model");;
    for triangle in lucy.triangles.into_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = prims.into_iter().collect();
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        background: Vec3 { x: 0.84, y: 0.34, z: 0.0 },
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
