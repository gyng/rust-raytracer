#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use mat4::{Mat4, Transform};
use raytracer::Octree;
use raytracer::animator::CameraKeyframe;
use scene::{Camera, Scene};
use vec3::Vec3;

// When using Fresnel, set k_sg and k_tg (if applicable) to 1.0 for easier material definition.
// You can still manually tweak it if you wish (try reducing k_sg for metals)

// 2500 polys, marginal improvement from an octree
pub fn get_teapot_camera(image_width: int, image_height: int, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: -0.2, y: 1.0, z: 2.0 },
        Vec3 { x: 0.0, y: 0.6, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_teapot_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(box SphereLight { position: Vec3 { x: 0.6, y: 2.0, z: 1.2 }, color: Vec3::one(), radius: 1.0 });

    let porcelain = CookTorranceMaterial { k_a: 0.0, k_d: 0.9, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.1, ior: 1.1, ambient: Vec3::one(), diffuse: Vec3 { x: 0.9, y: 0.85, z: 0.7 }, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None };

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    // prims.push(box Plane { a: 0.0, b: 1.0, c: 0.0, d: 0.0, material: box green });
    let mut teapot = ::util::import::from_obj(porcelain, false, "./docs/assets/models/teapot.obj");
    let rotate = Transform::new(Mat4::rotate_x_deg_matrix(1.0));
    teapot.mut_transform(&rotate);
    for triangle in teapot.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        octree: octree,
        background: Vec3 { x: 0.3, y: 0.5, z: 0.8 },
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
