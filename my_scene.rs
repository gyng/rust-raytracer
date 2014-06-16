use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
// use light::Lights::{PointLight, SphereLight}; // All lights
use light::lights::{SphereLight};
// use material::Materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial}; // All materials
use material::materials::{CookTorranceMaterial, PhongMaterial};
use scene::{Camera, Scene};
use vec3::Vec3;

pub fn get_camera(image_width: int, image_height: int) -> Camera {
    Camera::new(
        Vec3 {x: 50.0, y: 25.0, z: 150.0},
        Vec3 {x: 50.0, y: 50.0, z: 50.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        30.0,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let max_lights = 10;
    let max_prims = 1000;

    let mut lights: Vec<Box<Light:Share+Send>> = Vec::with_capacity(max_lights);
    // lights.push(box PointLight {position: Vec3 {x: 50.0, y: 20.0, z: 50.0}, color: Vec3::one()});
    lights.push(box SphereLight {position: Vec3 {x: 50.0, y: 80.0, z: 50.0}, color: Vec3::one(), radius: 10.0});

    let grey    = CookTorranceMaterial {k_a: 0.0, k_d: 1.0, k_s: 1.0, k_sg: 0.0, k_tg: 0.0, gauss_constant: 1.0, roughness: 0.15, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.6, y: 0.6, z: 0.6}, specular: Vec3::one(), transmission: Vec3::zero()};
    let blue    = CookTorranceMaterial {k_a: 0.0, k_d: 0.2, k_s: 0.8, k_sg: 0.0, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.1, ior: 1.3, ambient: Vec3::one(), diffuse: Vec3 {x: 0.1, y: 0.1, z: 1.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let red     = PhongMaterial {k_a: 0.0, k_d: 0.6, k_s: 0.4, k_sg: 0.3, k_tg: 0.0, shininess: 10.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 0.0, z: 0.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let green   = PhongMaterial {k_a: 0.0, k_d: 0.9, k_s: 0.1, k_sg: 0.1, k_tg: 0.0, shininess: 10.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 0.0, y: 1.0, z: 0.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let shiny   = PhongMaterial {k_a: 0.0, k_d: 0.5, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, shininess: 50.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let refract = PhongMaterial {k_a: 0.0, k_d: 0.0, k_s: 1.0, k_sg: 0.0, k_tg: 1.0, shininess: 40.0, ior: 3.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3::one(), transmission: Vec3 {x: 0.8, y: 0.8, z: 0.8}};

    let mut prims: Vec<Box<Prim:Share+Send>> = Vec::with_capacity(max_prims);
    prims.push(box Plane {a: 0.0,  b:  0.0, c: 1.0, d: 0.0,   material: box grey  }); // Ahead
    prims.push(box Plane {a: 0.0,  b: -1.0, c: 0.0, d: 100.0, material: box grey  }); // Bottom
    prims.push(box Plane {a: 0.0,  b:  1.0, c: 0.0, d: 0.0,   material: box grey  }); // Top
    prims.push(box Plane {a: 1.0,  b:  0.0, c: 0.0, d: 0.0,   material: box red   }); // Left
    prims.push(box Plane {a: -1.0, b:  0.0, c: 0.0, d: 100.0, material: box green }); // Right
    prims.push(box Sphere {center: Vec3 {x: 30.0, y: 15.0, z: 20.0}, radius: 15.0, material: box shiny});
    prims.push(box Sphere {center: Vec3 {x: 70.0, y: 17.0, z: 60.0}, radius: 17.0, material: box refract});
    prims.push(box Sphere {center: Vec3 {x: 50.0, y: 50.0, z: 20.0}, radius: 10.0, material: box blue});
    prims.push(box Sphere {center: Vec3 {x: 20.0, y: 13.0, z: 90.0}, radius: 13.0, material: box blue});
    prims.push(box Triangle::auto_normal(Vec3 {x: 15.0, y: 50.0, z: 40.0}, Vec3 {x: 35.0, y: 50.0, z: 35.0}, Vec3 {x: 20.0, y: 95.0, z: 20.0}, box blue));
    prims.push(box Triangle::auto_normal(
        Vec3 {x: 20.0, y: 95.0, z: 20.0},
        Vec3 {x: 15.0, y: 50.0, z: 40.0},
        Vec3 {x: 35.0, y: 50.0, z: 35.0},
        box blue));

    Scene {
        lights: lights,
        prims: prims,
        background: Vec3::one()
    }
}
