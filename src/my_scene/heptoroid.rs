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

// 114688 tris, 57302 verts
pub fn get_camera(image_width: int, image_height: int, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: 7.0, y: 2.0, z: -6.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene(material_option: &str) -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(box SphereLight { position: Vec3 { x: 2.0, y: 3.0, z: -2.0 }, color: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, radius: 1.0 });

    // Defaults to white
    let heptoroid_material = match material_option {
        "shiny" => CookTorranceMaterial { k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 0.55, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, ior: 0.25, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3::zero(), diffuse_texture: None },
        "refractive" => CookTorranceMaterial { k_a: 0.0, k_d: 0.0, k_s: 1.0, k_sg: 1.0, k_tg: 1.0, gauss_constant: 5.0, roughness: 0.01, ior: 1.50, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3 { x: 0.8, y: 0.8, z: 0.8 }, diffuse_texture: None },
        _ => CookTorranceMaterial { k_a: 0.0, k_d: 0.9, k_s: 1.0, k_sg: 0.15, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.1, ior: 0.5, ambient: Vec3::one(), diffuse: Vec3 { x: 0.9, y: 0.85, z: 0.7 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None }
    };

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    let heptoroid = ::util::import::from_obj(heptoroid_material, false, "./docs/assets/models/heptoroid.obj");
    for triangle in heptoroid.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        background: Vec3 { x: 0.84, y: 0.34, z: 0.0 },
        skybox: Some(CubeMap::load(
            "./docs/assets/textures/skyboxes/miramar_y_up/left.ppm",
            "./docs/assets/textures/skyboxes/miramar_y_up/right.ppm",
            "./docs/assets/textures/skyboxes/miramar_y_up/down.ppm",
            "./docs/assets/textures/skyboxes/miramar_y_up/up.ppm",
            "./docs/assets/textures/skyboxes/miramar_y_up/front.ppm",
            "./docs/assets/textures/skyboxes/miramar_y_up/back.ppm"
        ))
    }
}
