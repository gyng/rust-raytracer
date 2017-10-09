#![allow(unused_imports)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::animator::CameraKeyframe;
use raytracer::compositor::ColorRGBA;
use raytracer::animator::easing::Easing;
use scene::{Camera, Scene};
use vec3::Vec3;

// Easing test scene
pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: 0.0, y: 0.0, z: 150.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_animation_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    Camera::new_with_keyframes(
        Vec3 { x: 0.0, y: 0.0, z: 150.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height,
        vec![
            CameraKeyframe {
                time: 10.0,
                position: Vec3 { x: 0.0, y: 1000.0, z: 150.0 },
                look_at: Vec3 { x: 0.0, y: 1000.0, z: 0.0 },
                up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                easing: Easing { a: 0.0, b: 0.05, c: 0.1, d: 1.0 }
            },
        ]
    )
}

pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Sync>> = Vec::new();
    lights.push(Box::new(SphereLight {
        position: Vec3 { x: 0.0, y: 0.0, z: 150.0 },
        color: Vec3::one(),
        radius: 10.0
    }));

    lights.push(Box::new(SphereLight {
        position: Vec3 { x: 0.0, y: 1000.0, z: 150.0 },
        color: Vec3::one(),
        radius: 10.0
    }));

    let checker: Box<Texture+Send+Sync> = Box::new(CheckerTexture {
        color1: ColorRGBA::white(),
        color2: ColorRGBA::new_rgb(0.1, 0.1, 0.1),
        scale: 32.0
    });
    let checker_mat = CookTorranceMaterial {
        k_a: 0.0,
        k_d: 1.0,
        k_s: 0.0,
        k_sg: 0.0,
        k_tg: 0.0,
        gauss_constant: 1.0,
        roughness: 0.15,
        glossiness: 0.0,
        ior: 0.7,
        ambient: Vec3::one(),
        diffuse: Vec3 { x: 0.6, y: 0.6, z: 0.6 },
        specular: Vec3::one(),
        transmission: Vec3::zero(),
        diffuse_texture: Some(checker)
    };

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    prims.push(Box::new(Plane {
        a: 0.0,
        b: 0.0,
        c: 1.0,
        d: 0.0,
        material: Box::new(checker_mat)
    }));

    let octree = prims.into_iter().collect();

    Scene {
        lights: lights,
        octree: octree,
        background: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
        skybox: None
    }
}

pub struct EasingConfig;

impl super::SceneConfig for EasingConfig {
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