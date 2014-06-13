use vec3::Vec3;
use pointlight::PointLight;
use sphere::Sphere;
use flatmaterial::FlatMaterial;
use light::Light;
use prim::Prim;

mod vec3;
mod ray;
mod camera;
mod prim;
mod sphere;
mod light;
mod pointlight;
mod material;
mod flatmaterial;
mod intersection;
mod scene;
mod renderer;
mod export;


fn main() {
    let image_width = 64;
    let image_height = 48;

    let max_lights = 10;
    let max_prims = 1000;

    let mut lights: Vec<Box<Light>> = Vec::with_capacity(max_lights);
    lights.push(box PointLight {position: Vec3 {x: 50.0, y: 90.0, z: 50.0}, color: Vec3 {x: 1.0, y: 1.0, z: 1.0}});

    let flat_material = box FlatMaterial {color: Vec3 {x: 1.0, y: 0.0, z: 0.0}};

    let mut prims: Vec<Box<Prim>> = Vec::with_capacity(max_prims);
    prims.push(box Sphere {center: Vec3 {x: 30.0, y: 15.0, z: 20.0}, radius: 15.0, material: flat_material});

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
        background: Vec3 {x: 0.1, y: 0.1, z: 0.3}
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
