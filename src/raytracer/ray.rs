use std::f64::INFINITY;
use raytracer::Intersection;
use scene::Scene;
use vec3::Vec3;

#[cfg(test)]
use geometry::prim::Prim;
#[cfg(test)]
use geometry::prims::Sphere;
#[cfg(test)]
use light::light::Light;
#[cfg(test)]
use material::materials::FlatMaterial;
#[cfg(test)]
use raytracer::Octree;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub inverse_dir: Vec3, // This is used to optimise ray-bbox intersection checks
    pub signs: Vec<bool> // Handle degenerate case in bbox intersection
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        let inv_x = 1.0 / direction.x;
        let inv_y = 1.0 / direction.y;
        let inv_z = 1.0 / direction.z;

        Ray {
            origin: origin,
            direction: direction,
            inverse_dir: Vec3 {
                x: inv_x,
                y: inv_y,
                z: inv_z
            },
            signs: vec![
                inv_x > 0.0,
                inv_y > 0.0,
                inv_z > 0.0
            ]
        }
    }

    pub fn get_nearest_hit<'a>(&'a self, scene: &'a Scene) -> Option<Intersection<'a>> {
        let t_min = 0.000001;
        let mut nearest_hit = None;
        let mut nearest_t = INFINITY;

        for prim in scene.octree.get_intersected_objects(self) {
            let intersection = prim.intersects(self, t_min, nearest_t);

            nearest_hit = match intersection {
                Some(intersection) => {
                    if intersection.t > t_min && intersection.t < nearest_t {
                        nearest_t = intersection.t;
                        Some(intersection)
                    } else {
                        nearest_hit
                    }
                },
                None => nearest_hit
            };
        }

        nearest_hit
    }
}

#[test]
fn it_gets_the_nearest_hit() {
    let lights: Vec<Box<Light+Send+Sync>> = Vec::new();

    let mut prims: Vec<Box<Prim+Send+Sync>> = Vec::new();
    let mat = FlatMaterial { color: Vec3::one() };
    let sphere_top = Sphere {
        center: Vec3::zero(),
        radius: 1.0,
        material: Box::new(mat.clone()),
    };
    let sphere_mid = Sphere {
        center: Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        radius: 1.0,
        material: Box::new(mat.clone()),
    };
    let sphere_bot = Sphere {
        center: Vec3 { x: -2.0, y: 0.0, z: 0.0 },
        radius: 1.0,
        material: Box::new(mat.clone()),
    };
    prims.push(Box::new(sphere_top));
    prims.push(Box::new(sphere_mid));
    prims.push(Box::new(sphere_bot));

    println!("Generating octree...");
    let octree = Octree::new_from_prims(prims);
    println!("Octree generated...");

    let scene = Scene {
        lights: lights,
        background: Vec3::one(),
        octree: octree,
        skybox: None
    };

    let intersecting_ray = Ray::new(
        Vec3 { x: 10.0, y: 0.0, z: 0.0 },
        Vec3 { x: -1.0, y: 0.0, z: 0.0 }
    );

    let intersection = intersecting_ray.get_nearest_hit(&scene);
    assert_eq!(1.0, intersection.unwrap().position.x);

    let non_intersecting_ray = Ray::new(
        Vec3 { x: 10.0, y: 0.0, z: 0.0 },
        Vec3 { x: 1.0, y: 0.0, z: 0.0 });

    let non_intersection = non_intersecting_ray.get_nearest_hit(&scene);
    assert!(non_intersection.is_none());
}
