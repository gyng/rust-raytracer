use geometry::prim::{Prim};
use geometry::prims::{Plane, Sphere, Triangle};
use light::light::{Light};
use light::lights::{SphereLight};
use material::materials::{CookTorranceMaterial, PhongMaterial};
use raytracer::Octree;
use scene::{Camera, Scene};
use vec3::Vec3;
// use light::Lights::{PointLight, SphereLight}; // All lights
// use material::Materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial}; // All materials

// 10 primitives, octree is super inefficient for this scene
#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn get_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    // lights.push(box PointLight {position: Vec3 {x: 50.0, y: 20.0, z: 50.0}, color: Vec3::one()});
    lights.push(box SphereLight {position: Vec3 {x: 50.0, y: 80.0, z: 50.0}, color: Vec3::one(), radius: 10.0});

    let grey    = CookTorranceMaterial {k_a: 0.0, k_d: 1.0, k_s: 1.0, k_sg: 0.0, k_tg: 0.0, gauss_constant: 1.0, roughness: 0.15, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.6, y: 0.6, z: 0.6}, specular: Vec3::one(), transmission: Vec3::zero()};
    let blue    = CookTorranceMaterial {k_a: 0.0, k_d: 0.2, k_s: 0.8, k_sg: 0.0, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.1, ior: 1.3, ambient: Vec3::one(), diffuse: Vec3 {x: 0.1, y: 0.1, z: 1.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let red     = PhongMaterial {k_a: 0.0, k_d: 0.6, k_s: 0.4, k_sg: 0.3, k_tg: 0.0, shininess: 10.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 0.0, z: 0.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let green   = PhongMaterial {k_a: 0.0, k_d: 0.9, k_s: 0.1, k_sg: 0.1, k_tg: 0.0, shininess: 10.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 0.0, y: 1.0, z: 0.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let shiny   = PhongMaterial {k_a: 0.0, k_d: 0.5, k_s: 1.0, k_sg: 1.0, k_tg: 0.0, shininess: 50.0, ior: 1.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3::one(), transmission: Vec3::zero()};
    let refract = PhongMaterial {k_a: 0.0, k_d: 0.0, k_s: 1.0, k_sg: 0.0, k_tg: 1.0, shininess: 40.0, ior: 3.0, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 1.0, z: 1.0}, specular: Vec3::one(), transmission: Vec3 {x: 0.8, y: 0.8, z: 0.8}};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0,  b:  0.0, c: 1.0, d: 0.0,   material: box grey  }); // Ahead
    prims.push(box Plane {a: 0.0,  b: -1.0, c: 0.0, d: 100.0, material: box grey  }); // Bottom
    prims.push(box Plane {a: 0.0,  b:  1.0, c: 0.0, d: 0.0,   material: box grey  }); // Top
    prims.push(box Plane {a: 1.0,  b:  0.0, c: 0.0, d: 0.0,   material: box red   }); // Left
    prims.push(box Plane {a: -1.0, b:  0.0, c: 0.0, d: 100.0, material: box green }); // Right
    prims.push(box Sphere {center: Vec3 {x: 30.0, y: 15.0, z: 20.0}, radius: 15.0, material: box shiny});
    prims.push(box Sphere {center: Vec3 {x: 70.0, y: 17.0, z: 60.0}, radius: 17.0, material: box refract});
    prims.push(box Sphere {center: Vec3 {x: 50.0, y: 50.0, z: 20.0}, radius: 10.0, material: box blue});
    prims.push(box Sphere {center: Vec3 {x: 20.0, y: 13.0, z: 90.0}, radius: 13.0, material: box blue});
    prims.push(box Triangle::auto_normal(Vec3 {x: 20.0, y: 95.0, z: 20.0}, Vec3 {x: 15.0, y: 50.0, z: 40.0}, Vec3 {x: 35.0, y: 50.0, z: 35.0}, box blue));

    // Not complex enough to benefit from an octree
    // println!("Generating octree...");
    // let octree = Octree::new_from_prims(&prims);
    // println!("Octree generated...");

    Scene {
        lights: lights,
        prims: prims,
        background: Vec3::one(),
        octree: None
    }
}

// 300 polys, octree is slightly slower than no octree
#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn get_bunny_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 200.0, y: -200.0, z: 100.0}, color: Vec3::one(), radius: 40.0});
    lights.push(box SphereLight {position: Vec3 {x: -95.0, y: 20.0, z: 170.0}, color: Vec3{x: 0.5, y: 0.5, z: 0.3}, radius: 15.0});

    let red   = CookTorranceMaterial {k_a: 0.0, k_d: 0.4, k_s: 0.5, k_sg: 0.6, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.1, ior: 1.3, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 0.25, z: 0.1}, specular: Vec3::one(), transmission: Vec3::zero()};
    let green = CookTorranceMaterial {k_a: 0.0, k_d: 0.5, k_s: 0.5, k_sg: 0.2, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.3, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.2, y: 0.7, z: 0.2}, specular: Vec3::one(), transmission: Vec3::zero()};
    let shiny = CookTorranceMaterial {k_a: 0.0, k_d: 0.2, k_s: 0.5, k_sg: 0.8, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.1, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.9, y: 0.9, z: 0.1}, specular: Vec3 {x: 0.9, y: 0.9, z: 0.1}, transmission: Vec3::zero()};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0, b: 0.0, c: 1.0, d: -10.0, material: box green});
    prims.push(box Sphere {center: Vec3 {x: -75.0, y: 60.0, z: 50.0}, radius: 40.0, material: box shiny});
    prims.push(box Sphere {center: Vec3 {x: -75.0, y: 60.0, z: 140.0}, radius: 40.0, material: box shiny});
    let bunny = ::util::import::from_obj(Vec3::zero(), 1.0, red, false, "./docs/models/bunny.obj");
    for triangle in bunny.triangles.move_iter() { prims.push(triangle); }

    // Not complex enough to benefit from an octree
    // println!("Generating octree...");
    // let octree = Octree::new_from_prims(&prims);
    // println!("Octree generated...");

    Scene {
        lights: lights,
        prims: prims,
        background: Vec3 {x: 0.3, y: 0.5, z: 0.8},
        octree: None
    }
}

// 2500 polys, marginal improvement from an octree
#[allow(dead_code)]
pub fn get_teapot_camera(image_width: int, image_height: int) -> Camera {
    Camera::new(
        Vec3 {x: -2.0, y: 7.0, z: 10.0},
        Vec3 {x: 0.0, y: 3.0, z: 0.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        30.0,
        image_width,
        image_height
    )
}

#[allow(dead_code)]
pub fn get_teapot_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 3.0, y: 10.0, z: 6.0}, color: Vec3::one(), radius: 5.0});
    // lights.push(box SphereLight {position: Vec3 {x: -95.0, y: 20.0, z: 170.0}, color: Vec3{x: 0.5, y: 0.5, z: 0.3}, radius: 15.0});

    let red   = CookTorranceMaterial {k_a: 0.0, k_d: 0.4, k_s: 0.5, k_sg: 0.4, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.1, ior: 1.3, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 0.25, z: 0.1}, specular: Vec3::one(), transmission: Vec3::zero()};
    let green = CookTorranceMaterial {k_a: 0.0, k_d: 0.5, k_s: 0.5, k_sg: 0.2, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.3, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.2, y: 0.7, z: 0.2}, specular: Vec3::one(), transmission: Vec3::zero()};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0, b: 1.0, c: 0.0, d: 0.0, material: box green});
    let teapot = ::util::import::from_obj(Vec3::zero(), 5.0, red, false, "./docs/models/teapot.obj");
    for triangle in teapot.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(&prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prims: prims,
        background: Vec3 {x: 0.3, y: 0.5, z: 0.8},
        octree: Some(octree)
    }
}

// 5000 polys, cow. Octree helps.
#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn get_cow_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: 3.0, y: 10.0, z: 6.0}, color: Vec3::one(), radius: 5.0});

    let red   = CookTorranceMaterial {k_a: 0.0, k_d: 1.0, k_s: 0.5, k_sg: 0.3, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.1, ior: 1.3, ambient: Vec3::one(), diffuse: Vec3 {x: 1.0, y: 0.25, z: 0.1}, specular: Vec3::one(), transmission: Vec3::zero()};
    let green = CookTorranceMaterial {k_a: 0.0, k_d: 0.5, k_s: 0.5, k_sg: 0.2, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.3, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.2, y: 0.7, z: 0.2}, specular: Vec3::one(), transmission: Vec3::zero()};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0, b: 1.0, c: 0.0, d: 3.6, material: box green});
    let cow = ::util::import::from_obj(Vec3::zero(), 1.0, red, true, "./docs/models/cow.obj");
    for triangle in cow.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(&prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prims: prims,
        background: Vec3 {x: 0.3, y: 0.5, z: 0.8},
        octree: Some(octree)
    }
}

// 50000 polys, model not included!
#[allow(dead_code)]
pub fn get_lucy_camera(image_width: int, image_height: int) -> Camera {
    Camera::new(
        Vec3 {x: -1500.0, y: 300.0, z: 100.0},
        Vec3 {x: 0.0, y: 400.0, z: 0.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        30.0,
        image_width,
        image_height
    )
}

#[allow(dead_code)]
pub fn get_lucy_scene() -> Scene {
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();
    lights.push(box SphereLight {position: Vec3 {x: -1400.0, y: 200.0, z: 100.0}, color: Vec3::one(), radius: 5.0});
    // lights.push(box SphereLight {position: Vec3 {x: -95.0, y: 20.0, z: 170.0}, color: Vec3{x: 0.5, y: 0.5, z: 0.3}, radius: 15.0});

    let grey  = CookTorranceMaterial {k_a: 0.0, k_d: 0.5, k_s: 0.5, k_sg: 0.5, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.1, ior: 1.3, ambient: Vec3::one(), diffuse: Vec3 {x: 0.6, y: 0.6, z: 0.65}, specular: Vec3::one(), transmission: Vec3::zero()};
    let green = CookTorranceMaterial {k_a: 0.0, k_d: 0.5, k_s: 0.5, k_sg: 0.2, k_tg: 0.0, gauss_constant: 50.0, roughness: 0.3, ior: 1.5, ambient: Vec3::one(), diffuse: Vec3 {x: 0.2, y: 0.7, z: 0.2}, specular: Vec3::one(), transmission: Vec3::zero()};

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    prims.push(box Plane {a: 0.0, b: 1.0, c: 0.0, d: 600.0, material: box green});
    let lucy = ::util::import::from_obj(Vec3::zero(), 1.0, grey, true, "./docs/models/lucy.obj");
    for triangle in lucy.triangles.move_iter() { prims.push(triangle); }

    println!("Generating octree...");
    let octree = Octree::new_from_prims(&prims);
    println!("Octree generated...");

    Scene {
        lights: lights,
        prims: prims,
        background: Vec3 {x: 0.3, y: 0.5, z: 0.8},
        octree: Some(octree)
    }
}

