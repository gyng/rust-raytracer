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

pub fn get_camera(image_width: int, image_height: int, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: 100.0, y: 60.0, z: -150.0 },
        Vec3 { x: 0.0, y: 50.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight { position: Vec3 { x: 0.0, y: 100.0, z: 0.0 }, color: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, radius: 25.0 });

    let blue =  CookTorranceMaterial { k_a: 0.0, k_d: 0.9, k_s: 1.0, k_sg: 0.4, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, ior: 0.25, ambient: Vec3::one(), diffuse: Vec3 { x: 0.16, y: 0.29, z: 0.44 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };
    let floor = CookTorranceMaterial { k_a: 0.0, k_d: 0.9, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.3, ior: 1.0,   ambient: Vec3::one(), diffuse: Vec3 { x: 0.58, y: 0.63, z: 0.44 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3::zero(), diffuse_texture: None };

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane { a: 0.0, b: 1.0, c: 0.0, d: 0.0, material: box floor.clone() }); // Bottom

    let tachikoma = ::util::import::from_obj(blue, false, "./docs/assets/models/tachikoma.obj");
    for triangle in tachikoma.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prim_strat: box octree,
        background: Vec3 { x: 0.2, y: 0.2, z: 0.2 },
        // skybox: None
        skybox: Some(CubeMap::load(
            "./docs/assets/textures/skyboxes/city_y_up/left.ppm",
            "./docs/assets/textures/skyboxes/city_y_up/right.ppm",
            "./docs/assets/textures/skyboxes/city_y_up/down.ppm",
            "./docs/assets/textures/skyboxes/city_y_up/up.ppm",
            "./docs/assets/textures/skyboxes/city_y_up/front.ppm",
            "./docs/assets/textures/skyboxes/city_y_up/back.ppm"
        ))
    }
}
