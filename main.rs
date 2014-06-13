use vec3::Vec3;
use pointlight::PointLight;
use sphere::Sphere;
use plane::Plane;
use flatmaterial::FlatMaterial;
use light::Light;
use prim::Prim;

mod vec3;
mod ray;
mod camera;
mod prim;
mod sphere;
mod plane;
mod light;
mod pointlight;
mod material;
mod flatmaterial;
mod intersection;
mod scene;
mod renderer;
mod export;

fn main() {
    let image_width = 50;
    let image_height = 50;

    let max_lights = 10;
    let max_prims = 1000;

    let mut lights: Vec<Box<Light>> = Vec::with_capacity(max_lights);
    lights.push(box PointLight {position: Vec3 {x: 50.0, y: 90.0, z: 50.0}, color: Vec3 {x: 1.0, y: 1.0, z: 1.0}});

    let grey1 = box FlatMaterial {color: Vec3 {x: 0.6, y: 0.6, z: 0.6}};
    let grey2 = box FlatMaterial {color: Vec3 {x: 0.6, y: 0.6, z: 0.6}};
    let grey3 = box FlatMaterial {color: Vec3 {x: 0.6, y: 0.6, z: 0.6}};
    let red   = box FlatMaterial {color: Vec3 {x: 1.0, y: 0.0, z: 0.0}};
    let green = box FlatMaterial {color: Vec3 {x: 0.0, y: 1.0, z: 0.0}};

    let blue1   = box FlatMaterial {color: Vec3 {x: 0.0, y: 0.0, z: 1.0}};
    let blue2   = box FlatMaterial {color: Vec3 {x: 0.3, y: 0.3, z: 1.0}};

    let mut prims: Vec<Box<Prim>> = Vec::with_capacity(max_prims);
    prims.push(box Plane {a: 0.0,  b:  0.0, c: 1.0, d: 0.0,   material: grey1 }); // Ahead
    prims.push(box Plane {a: 0.0,  b: -1.0, c: 0.0, d: 100.0, material: grey2 }); // Bottom
    prims.push(box Plane {a: 0.0,  b:  1.0, c: 0.0, d: 0.0,   material: grey3 }); // Top
    prims.push(box Plane {a: 1.0,  b:  0.0, c: 0.0, d: 0.0,   material: red   }); // Left
    prims.push(box Plane {a: -1.0, b:  0.0, c: 0.0, d: 100.0, material: green }); // Right

    prims.push(box Sphere {center: Vec3 {x: 30.0, y: 15.0, z: 20.0}, radius: 15.0, material: blue1});
    prims.push(box Sphere {center: Vec3 {x: 70.0, y: 17.0, z: 80.0}, radius: 17.0, material: blue2});

    let camera = camera::Camera::new(
        Vec3 {x: 50.0, y: 25.0, z: 300.0},
        Vec3 {x: 50.0, y: 50.0, z: 50.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        45.0,
        image_width,
        image_height
    );

    let scene = scene::Scene {
        lights: lights,
        prims: prims,
        background: Vec3 {x: 1.0, y: 1.0, z: 1.0}
    };

    let renderer = renderer::Renderer {
        reflect_depth: 2,
        refract_depth: 4,
        use_octree: false,
        shadows: false,
        threads: 1
    };
    let image_data = renderer.render(camera, scene);

    ::export::to_ppm(image_data, image_width, image_height, "test.ppm");
}
