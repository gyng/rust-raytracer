#![allow(dead_code)]

use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, PhongMaterial};
use material::Texture;
use material::textures::CheckerTexture;
use material::textures::CubeMap;
use raytracer::Octree;
use raytracer::VecPrimContainer;
use raytracer::animator::CameraKeyframe;
use scene::{Camera, Scene};
use vec3::Vec3;

// use light::Lights::{PointLight, SphereLight}; // All lights
// use material::Materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial}; // All materials
// use material::material::{CheckerTexture, UVTexture}; // All textures


// When using Fresnel, set k_sg and k_tg (if applicable) to 1.0 for easier material definition.
// You can still manually tweak it if you wish.


// 10 primitives, octree is super inefficient for this scene
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
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 50.0, y: 80.0, z: 50.0}, color: Vec3::one(), radius: 10.0});

    let checker: Box<Texture+Send+Share> = box CheckerTexture{color1: Vec3::one(), color2: Vec3 {x: 0.8, y: 0.1, z: 0.1}, scale: 16.0};
    // let wood: Box<Texture+Send+Share> = box ImageTexture {image: ::util::import::from_ppm("./docs/models/wood.ppm")};
    // let wood_mat     = CookTorranceMaterial {k_a: 0.0, k_d: 1.0, k_s: 1.0, k_sg: 0.0, k_tg: 0.0, gauss_constant: 1.0, roughness: 0.15, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.6, y: 0.6, z: 0.6}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: Some(wood.clone())};
    let checker_grey = CookTorranceMaterial {k_a: 0.0, k_d: 1.0, k_s: 1.0, k_sg: 0.0, k_tg: 0.0, gauss_constant: 1.0, roughness: 0.15, ior: 1.5,  ambient: Vec3::one(), diffuse: Vec3 {x: 0.6, y: 0.6, z: 0.6}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: Some(checker.clone())};
    let grey         = CookTorranceMaterial {k_a: 0.0, k_d: 1.0, k_s: 1.0, k_sg: 0.0, k_tg: 0.0, gauss_constant: 1.0, roughness: 0.15, ior: 1.5,  ambient: Vec3::one(), diffuse: Vec3 {x: 0.6, y: 0.6, z: 0.6}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let blue         = CookTorranceMaterial {k_a: 0.0, k_d: 0.3, k_s: 0.7, k_sg: 0.0, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.1, ior: 1.3,  ambient: Vec3::one(), diffuse: Vec3 {x: 0.1, y: 0.1, z: 1.0}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let red          = PhongMaterial        {k_a: 0.0, k_d: 0.6, k_s: 0.4, k_sg: 1.0, k_tg: 0.0, shininess: 10.0,                      ior: 0.25, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 0.0, z: 0.0}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let green        = PhongMaterial        {k_a: 0.0, k_d: 0.9, k_s: 0.1, k_sg: 0.5, k_tg: 0.0, shininess: 10.0,                      ior: 0.4,  ambient: Vec3::one(), diffuse: Vec3 {x: 0.0, y: 1.0, z: 0.0}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let shiny        = CookTorranceMaterial {k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, ior: 0.15, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3 {x: 0.9, y: 0.9, z: 0.9}, transmission: Vec3::zero(), diffuse_texture: None};
    let refract      = CookTorranceMaterial {k_a: 0.0, k_d: 0.0, k_s: 1.0, k_sg: 1.0, k_tg: 1.0, gauss_constant: 5.0, roughness: 0.01, ior: 3.0,  ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3 {x: 0.9, y: 0.9, z: 0.9}, transmission: Vec3 {x: 0.8, y: 0.8, z: 0.8}, diffuse_texture: None};
    // let refract      = CookTorranceMaterial {k_a: 0.0, k_d: 0.0, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, ior: 3.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3 {x: 0.9, y: 0.9, z: 0.9}, transmission: Vec3::zero(), diffuse_texture: None};
    // let refract      = PhongMaterial        {k_a: 0.0, k_d: 0.0, k_s: 1.0, k_sg: 1.0, k_tg: 1.0, shininess: 40.0,                      ior: 2.4, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3::one(), transmission: Vec3 {x: 0.8, y: 0.8, z: 0.8}, diffuse_texture: None};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0,  b:  0.0, c: 1.0, d: 0.0,   material: box grey.clone()         }); // Ahead
    prims.push(box Plane {a: 0.0,  b:  1.0, c: 0.0, d: 0.0,   material: box checker_grey.clone() }); // Bottom
    prims.push(box Plane {a: 0.0,  b: -1.0, c: 0.0, d: 100.0, material: box grey.clone()         }); // Top
    prims.push(box Plane {a: 1.0,  b:  0.0, c: 0.0, d: 0.0,   material: box red.clone()          }); // Left
    prims.push(box Plane {a: -1.0, b:  0.0, c: 0.0, d: 100.0, material: box green.clone()        }); // Right
    prims.push(box Sphere {center: Vec3 {x: 30.0, y: 15.0, z: 20.0}, radius: 15.0, material: box shiny.clone()});
    prims.push(box Sphere {center: Vec3 {x: 70.0, y: 17.0, z: 60.0}, radius: 17.0, material: box refract.clone()});
    prims.push(box Sphere {center: Vec3 {x: 50.0, y: 50.0, z: 20.0}, radius: 10.0, material: box blue.clone()});
    prims.push(box Sphere {center: Vec3 {x: 20.0, y: 13.0, z: 90.0}, radius: 13.0, material: box blue.clone()});
    prims.push(box Triangle::auto_normal(Vec3 {x: 20.0, y: 95.0, z: 20.0}, Vec3 {x: 15.0, y: 50.0, z: 40.0}, Vec3 {x: 35.0, y: 50.0, z: 35.0}, Vec3 {x: 0.5, y: 0.0, z: 1.0}, Vec3 {x: 1.0, y: 0.0, z: 0.0}, box blue));

    // Not complex enough to benefit from an octree
    // println!("Generating octree...");
    // let octree = Octree::new_from_prims(prims);
    // println!("Octree generated...");

    Scene {
        lights: lights,
        prim_strat: box VecPrimContainer::new(prims),
        background: Vec3::one(),
        skybox: None
    }
}

// 300 polys, octree is slightly slower than no octree
pub fn get_bunny_camera(image_width: int, image_height: int) -> Camera {
    Camera::new(
        Vec3 {x: 0.0, y: -150.0, z: 30.0},
        Vec3 {x: 0.0, y: 60.0, z: 50.0},
        Vec3 {x: 0.0, y: 0.0, z: 1.0},
        30.0,
        image_width,
        image_height
    )
}

pub fn get_bunny_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 200.0, y: -200.0, z: 100.0}, color: Vec3::one(), radius: 40.0});
    lights.push(box SphereLight {position: Vec3 {x: -95.0, y: 20.0, z: 170.0}, color: Vec3{x: 0.5, y: 0.5, z: 0.3}, radius: 15.0});

    let red   = CookTorranceMaterial {k_a: 0.1, k_d: 0.4, k_s: 0.5, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.05, ior: 0.28, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 0.25, z: 0.1}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let green = CookTorranceMaterial {k_a: 0.0, k_d: 0.4, k_s: 0.6, k_sg: 0.7, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.3, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.2, y: 0.7, z: 0.2}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let shiny = CookTorranceMaterial {k_a: 0.0, k_d: 0.2, k_s: 0.7, k_sg: 1.0, k_tg: 0.0, gauss_constant: 25.0, roughness: 0.01, ior: 0.1, ambient: Vec3::one(), diffuse: Vec3 {x: 0.9, y: 0.9, z: 0.1}, specular: Vec3 {x: 0.9, y: 0.9, z: 0.1}, transmission: Vec3::zero(), diffuse_texture: None};


    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0, b: 0.0, c: 1.0, d: -10.0, material: box green});
    prims.push(box Sphere {center: Vec3 {x: -75.0, y: 60.0, z: 50.0}, radius: 40.0, material: box shiny.clone()});
    prims.push(box Sphere {center: Vec3 {x: -75.0, y: 60.0, z: 140.0}, radius: 40.0, material: box shiny.clone()});
    let bunny = ::util::import::from_obj(Vec3::zero(), 1.0, red, false, "./docs/assets/models/bunny.obj");
    for triangle in bunny.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prim_strat: box octree,
        background: Vec3 {x: 0.3, y: 0.5, z: 0.8},
        skybox: Some(CubeMap::load(
            "./docs/assets/textures/skyboxes/storm_z_up/left.ppm",
            "./docs/assets/textures/skyboxes/storm_z_up/right.ppm",
            "./docs/assets/textures/skyboxes/storm_z_up/back.ppm",
            "./docs/assets/textures/skyboxes/storm_z_up/front.ppm",
            "./docs/assets/textures/skyboxes/storm_z_up/down.ppm",
            "./docs/assets/textures/skyboxes/storm_z_up/up.ppm"
        ))
    }
}

// 2500 polys, marginal improvement from an octree
pub fn get_teapot_camera(image_width: int, image_height: int) -> Camera {
    Camera::new(
        Vec3 {x: -2.0, y: 5.0, z: 10.0},
        Vec3 {x: 0.0, y: 3.0, z: 0.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        30.0,
        image_width,
        image_height
    )
}

pub fn get_teapot_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 3.0, y: 10.0, z: 6.0}, color: Vec3::one(), radius: 5.0});

    let porcelain = CookTorranceMaterial {k_a: 0.0, k_d: 0.9, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.1, ior: 0.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.9, y: 0.85, z: 0.7}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    // prims.push(box Plane {a: 0.0, b: 1.0, c: 0.0, d: 0.0, material: box green});
    let teapot = ::util::import::from_obj(Vec3::zero(), 5.0, porcelain, false, "./docs/assets/models/teapot.obj");
    for triangle in teapot.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prim_strat: box octree,
        background: Vec3 {x: 0.3, y: 0.5, z: 0.8},
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

// 5000 polys, cow. Octree helps.
pub fn get_cow_camera(image_width: int, image_height: int) -> Camera {
    Camera::new(
        Vec3 {x: -2.0, y: 4.0, z: 10.0},
        Vec3 {x: 0.0, y: 0.0, z: 0.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        30.0,
        image_width,
        image_height
    )
}

pub fn get_cow_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 3.0, y: 10.0, z: 6.0}, color: Vec3::one(), radius: 5.0});

    let red   = CookTorranceMaterial {k_a: 0.0, k_d: 0.6, k_s: 0.6, k_sg: 0.6, k_tg: 0.0, gauss_constant: 15.0, roughness: 0.05, ior: 0.28, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 0.25, z: 0.1}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let green = CookTorranceMaterial {k_a: 0.0, k_d: 0.5, k_s: 0.4, k_sg: 0.4, k_tg: 0.0, gauss_constant: 25.0, roughness: 0.4,  ior: 0.5,  ambient: Vec3::one(), diffuse: Vec3 {x: 0.2, y: 0.7, z: 0.2},  specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0, b: 1.0, c: 0.0, d: 3.6, material: box green});
    let cow = ::util::import::from_obj(Vec3::zero(), 1.0, red, true, "./docs/assets/models/cow.obj");
    for triangle in cow.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prim_strat: box octree,
        background: Vec3 {x: 0.3, y: 0.5, z: 0.8},
        skybox: None
    }
}

// 50000 polys, model not included!
pub fn get_lucy_camera(image_width: int, image_height: int) -> Camera {
    Camera::new(
        Vec3 {x: -1500.0, y: 300.0, z: 600.0},
        Vec3 {x: 0.0, y: 400.0, z: -200.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        30.0,
        image_width,
        image_height
    )
}

pub fn get_lucy_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: -1400.0, y: 200.0, z: 100.0}, color: Vec3 {x: 1.0, y: 0.80, z: 0.40}, radius: 50.0});

    let grey = CookTorranceMaterial {k_a: 0.0, k_d: 0.5, k_s: 0.6, k_sg: 0.8, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.1, ior: 0.2, ambient: Vec3::one(), diffuse: Vec3 {x: 0.6, y: 0.6, z: 0.65}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    // let ground = CookTorranceMaterial {k_a: 0.0, k_d: 0.5, k_s: 0.5, k_sg: 0.2, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.3, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.43, y: 0.38, z: 0.33}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    // prims.push(box Plane {a: 0.0, b: 1.0, c: 0.0, d: 600.0, material: box ground});
    let lucy = ::util::import::from_obj(Vec3::zero(), 1.0, grey, true, "./docs/assets/models/lucy.obj");
    for triangle in lucy.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prim_strat: box octree,
        background: Vec3 {x: 0.84, y: 0.34, z: 0.0},
        skybox: Some(CubeMap::load(
            "./docs/assets/textures/skyboxes/storm_y_up/left.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/right.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/down.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/up.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/front.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/back.ppm"
        ))
    }
}


// ~28000 triangles, complex scene with 2 lights
pub fn get_sponza_camera(image_width: int, image_height: int) -> Camera {
    Camera::new(
        Vec3 {x: 800.0, y: 30.0, z: 90.0},
        Vec3 {x: -500.0, y: 1000.0, z: -100.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        45.0,
        image_width,
        image_height
    )
}

pub fn get_sponza_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 0.0, y: 3000.0, z: 1000.0}, color: Vec3 {x: 1.0, y: 0.8, z: 0.4}, radius: 50.0});
    lights.push(box SphereLight {position: Vec3 {x: 300.0, y: 300.0, z: 60.0}, color: Vec3 {x: 0.38, y: 0.32, z: 0.28}, radius: 20.0});

    let checker: Box<Texture+Send+Share> = box CheckerTexture{color1: Vec3::one(), color2: Vec3 {x: 0.15, y: 0.11, z: 0.1}, scale: 32.0};

    let stone     = CookTorranceMaterial {k_a: 0.1, k_d: 0.8, k_s: 0.2, k_sg: 0.2, k_tg: 0.0, gauss_constant: 50.0, roughness: 1.0, ior: 0.7, ambient: Vec3 {x: 0.88, y: 0.83, z: 0.77}, diffuse: Vec3 {x: 0.88, y: 0.83, z: 0.77}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let ground    = CookTorranceMaterial {k_a: 0.03, k_d: 0.9, k_s: 0.3, k_sg: 0.5, k_tg: 0.0, gauss_constant: 25.0, roughness: 0.1, ior: 0.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.38, y: 0.38, z: 0.5}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: Some(checker.clone())};
    let cloth     = CookTorranceMaterial {k_a: 0.03, k_d: 0.8, k_s: 0.1, k_sg: 0.05, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.8, ior: 1.3, ambient: Vec3::one(), diffuse: Vec3 {x: 0.85, y: 0.05, z: 0.05}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let shrubbery = CookTorranceMaterial {k_a: 0.03, k_d: 0.8, k_s: 0.2, k_sg: 0.05, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.2, ior: 1.2, ambient: Vec3::one(), diffuse: Vec3 {x: 0.16, y: 0.47, z: 0.11}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0, b: 1.0, c: 0.0, d: 0.0, material: box ground});

    let sponza_other = ::util::import::from_obj(Vec3::zero(), 1.0, stone, false, "./docs/assets/models/sponza_other.obj");
    for triangle in sponza_other.triangles.move_iter() { prims.push(triangle); }

    let sponza_column_shrubbery = ::util::import::from_obj(Vec3::zero(), 1.0, shrubbery, false, "./docs/assets/models/sponza_column_shrubbery.obj");
    for triangle in sponza_column_shrubbery.triangles.move_iter() { prims.push(triangle); }

    let sponza_cloth = ::util::import::from_obj(Vec3::zero(), 1.0, cloth, false, "./docs/assets/models/sponza_cloth.obj");
    for triangle in sponza_cloth.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prim_strat: box octree,
        background: Vec3 {x: 0.84, y: 0.34, z: 0.0},
        skybox: Some(CubeMap::load(
            "./docs/assets/textures/skyboxes/storm_y_up/left.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/right.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/down.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/up.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/front.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/back.ppm"
        ))
    }
}

// ~70K triangles, no textures yet
pub fn get_sibenik_camera(image_width: int, image_height: int) -> Camera {
    Camera::new(
        Vec3 {x: -16.0, y: -14.5, z: -2.0},
        Vec3 {x: 8.0, y: -3.0, z: 2.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        30.0,
        image_width,
        image_height
    )
}

pub fn get_sibenik_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 8.0, y: 8.0, z: 0.0}, color: Vec3 {x: 1.0, y: 0.8, z: 0.4}, radius: 0.5});
    lights.push(box SphereLight {position: Vec3 {x: 8.0, y: -5.0, z: 0.0}, color: Vec3 {x: 0.5, y: 0.4, z: 0.2}, radius: 1.0});
    lights.push(box PointLight {position: Vec3 {x: -16.0, y: -14.5, z: -2.0}, color: Vec3 {x: 0.15, y: 0.07, z: 0.05}});

    let checker: Box<Texture+Send+Share> = box CheckerTexture{color1: Vec3::one(), color2: Vec3 {x: 0.15, y: 0.11, z: 0.1}, scale: 1.0};

    let stone     = CookTorranceMaterial {k_a: 0.1, k_d: 0.8, k_s: 0.2, k_sg: 0.0, k_tg: 0.0, gauss_constant: 25.0, roughness: 1.0, ior: 0.7, ambient: Vec3 {x: 0.88, y: 0.83, z: 0.77}, diffuse: Vec3 {x: 0.88, y: 0.83, z: 0.77}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: None};
    let ground    = CookTorranceMaterial {k_a: 0.03, k_d: 0.9, k_s: 0.3, k_sg: 0.5, k_tg: 0.0, gauss_constant: 25.0, roughness: 0.1, ior: 0.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.38, y: 0.38, z: 0.5}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: Some(checker.clone())};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0,  b: -1.0, c: 0.0, d: -14.9,   material: box ground.clone() });

    let sibenik = ::util::import::from_obj(Vec3::zero(), 1.0, stone, false, "./docs/assets/models/sibenik.obj");
    for triangle in sibenik.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prim_strat: box octree,
        background: Vec3 {x: 0.5, y: 0.5, z: 0.5},
        skybox: None
    }
}

// Skybox test scene
pub fn get_sphere_camera(image_width: int, image_height: int) -> Camera {
    let up = Vec3 {x: 0.0, y: 1.0, z: 0.0}; // y-up
    // let up = Vec3 {x: 0.0, y: 0.0, z: 1.0}; // z-up

    Camera::new(
        Vec3 {x: 0.0, y: 0.0, z: 10.0},
        Vec3 {x: 0.0, y: 0.0, z: 0.0},
        up,
        30.0,
        image_width,
        image_height
    )
}

pub fn get_sphere_animation_camera(image_width: int, image_height: int) -> Camera {
    // State at time t=0
    // A keyframe at time t=0 is automatically created when insert_keyframes is called
    let camera = Camera::new_with_keyframes(
        Vec3 {x: 0.0, y: 0.0, z: 10.0},
        Vec3 {x: 0.0, y: 0.0, z: 0.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        30.0,
        image_width,
        image_height,
        vec![
            CameraKeyframe {
                time: 2.5,
                position: Vec3 {x: 10.0, y: 0.0, z: 0.0},
                look_at: Vec3 {x: 0.0, y: 0.0, z: 0.0},
                up: Vec3 {x: 0.0, y: 1.0, z: 0.0}
            },
            CameraKeyframe {
                time: 5.0,
                position: Vec3 {x: 0.0, y: 0.0, z: -10.0},
                look_at: Vec3 {x: 0.0, y: 0.0, z: 0.0},
                up: Vec3 {x: 0.0, y: 1.0, z: 0.0}
            },
            CameraKeyframe {
                time: 7.5,
                position: Vec3 {x: -10.0, y: 0.0, z: 0.0},
                look_at: Vec3 {x: 0.0, y: 0.0, z: 0.0},
                up: Vec3 {x: 0.0, y: 1.0, z: 0.0}
            },
            CameraKeyframe {
                time: 10.0,
                position: Vec3 {x: 0.0, y: 0.0, z: 10.0},
                look_at: Vec3 {x: 0.0, y: 0.0, z: 0.0},
                up: Vec3 {x: 0.0, y: 1.0, z: 0.0}
            },
        ]
    );

    camera
}

pub fn get_sphere_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 3.0, y: 10.0, z: 6.0}, color: Vec3::one(), radius: 5.0});

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    let shiny = CookTorranceMaterial {k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, ior: 0.05, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3 {x: 0.9, y: 0.9, z: 0.9}, transmission: Vec3::zero(), diffuse_texture: None};
    prims.push(box Sphere {center: Vec3::zero(), radius: 2.0, material: box shiny});

    // For y as up
    Scene {
        lights: lights,
        background: Vec3 {x: 0.3, y: 0.5, z: 0.8},
        prim_strat: box VecPrimContainer::new(prims),
        skybox: Some(CubeMap::load(
            "./docs/assets/textures/skyboxes/storm_y_up/left.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/right.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/down.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/up.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/front.ppm",
            "./docs/assets/textures/skyboxes/storm_y_up/back.ppm"
        ))
    }
}



// Fresnel test scene
pub fn get_fresnel_camera(image_width: int, image_height: int) -> Camera {
    let height = 50.0;

    Camera::new(
        Vec3 {x: 50.0, y: height, z: 250.0},
        Vec3 {x: 50.0, y: 50.0, z: 50.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        30.0,
        image_width,
        image_height
    )
}

pub fn get_fresnel_animation_camera(image_width: int, image_height: int) -> Camera {
    // State at time t=0
    // A keyframe at time t=0 is automatically created when insert_keyframes is called
    let camera = Camera::new_with_keyframes(
        Vec3 {x: 0.0, y: 1.0, z: 250.0},
        Vec3 {x: 0.0, y: 1.0, z: 50.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        45.0,
        image_width,
        image_height,
        vec![
            CameraKeyframe {
                time: 2.5,
                position: Vec3 {x: 50.0, y: 100.0, z: 250.0},
                look_at: Vec3 {x: 0.0, y: 1.0, z: 50.0},
                up: Vec3 {x: 0.0, y: 1.0, z: 0.0}
            },
            CameraKeyframe {
                time: 5.0,
                position: Vec3 {x: 0.0, y: 200.0, z: 250.0},
                look_at: Vec3 {x: 0.0, y: 1.0, z: 50.0},
                up: Vec3 {x: 0.0, y: 1.0, z: 0.0}
            },
            CameraKeyframe {
                time: 7.5,
                position: Vec3 {x: -50.0, y: 100.0, z: 250.0},
                look_at: Vec3 {x: 0.0, y: 1.0, z: 50.0},
                up: Vec3 {x: 0.0, y: 1.0, z: 0.0}
            },
            CameraKeyframe {
                time: 10.0,
                position: Vec3 {x: 0.0, y: 1.0, z: 250.0},
                look_at: Vec3 {x: 0.0, y: 1.0, z: 50.0},
                up: Vec3 {x: 0.0, y: 1.0, z: 0.0}
            },
        ]
    );

    camera
}

pub fn get_fresnel_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 50.0, y: 80.0, z: 50.0}, color: Vec3::one(), radius: 10.0});

    let checker: Box<Texture+Send+Share> = box CheckerTexture{color1: Vec3::one(), color2: Vec3 {x: 0.1, y: 0.1, z: 0.1}, scale: 32.0};
    let checker_red          = CookTorranceMaterial {k_a: 0.0, k_d: 1.0, k_s: 0.0, k_sg: 0.0, k_tg: 0.0, gauss_constant: 1.0, roughness: 0.15, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.6, y: 0.6, z: 0.6}, specular: Vec3::one(), transmission: Vec3::zero(), diffuse_texture: Some(checker.clone())};
    let shiny                = CookTorranceMaterial {k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, ior: 0.15, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3 {x: 0.9, y: 0.9, z: 0.9}, transmission: Vec3::zero(), diffuse_texture: None};
    let global_specular_only = CookTorranceMaterial {k_a: 0.0, k_d: 0.0, k_s: 0.0, k_sg: 1.0, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, ior: 0.8, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3 {x: 0.9, y: 0.9, z: 0.9}, transmission: Vec3::zero(), diffuse_texture: None};
    let refract              = CookTorranceMaterial {k_a: 0.0, k_d: 0.0, k_s: 1.0, k_sg: 1.0, k_tg: 1.0, gauss_constant: 5.0, roughness: 0.01, ior: 3.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3 {x: 0.9, y: 0.9, z: 0.9}, transmission: Vec3::zero(), diffuse_texture: None};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0,  b:  0.0, c: 1.0, d: 0.0,   material: box checker_red.clone() }); // Ahead
    prims.push(box Plane {a: 0.0,  b:  1.0, c: 0.0, d: 0.0,   material: box global_specular_only.clone() }); // Bottom
    prims.push(box Sphere {center: Vec3 {x: 30.0, y: 15.0, z: 20.0}, radius: 15.0, material: box shiny.clone()});
    prims.push(box Sphere {center: Vec3 {x: 70.0, y: 17.0, z: 60.0}, radius: 17.0, material: box refract.clone()});

    Scene {
        lights: lights,
        prim_strat: box VecPrimContainer::new(prims),
        background: Vec3{x: 1.0, y: 1.0, z: 1.0},
        skybox: None
    }
}
