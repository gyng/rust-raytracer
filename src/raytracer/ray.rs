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
use raytracer::VecPrimContainer;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub inverse_dir: Vec3 // This is used to optimise ray-bbox intersection checks
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            inverse_dir: Vec3 {
                x: 1.0 / direction.x,
                y: 1.0 / direction.y,
                z: 1.0 / direction.z
            }
        }
    }

    pub fn get_nearest_hit<'a>(&'a self, scene: &'a Scene) -> Option<Intersection<'a>> {
        let t_min = 0.000001;
        let mut nearest_hit = None;
        let mut nearest_t = INFINITY;

        let candidate_nodes = scene.prim_strat.get_intersection_objects(self);

        for prim in candidate_nodes.iter() {
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
    let mut lights: Vec<Box<Light+Send+Share>> = Vec::new();

    let mut prims: Vec<Box<Prim+Send+Share>> = Vec::new();
    let mat = FlatMaterial { color: Vec3::one() };
    let sphere_top = Sphere {
        center: Vec3::zero(),
        radius: 1.0,
        material: box mat.clone()
    };
    let sphere_mid = Sphere {
        center: Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        radius: 1.0,
        material: box mat.clone()
    };
    let sphere_bot = Sphere {
        center: Vec3 { x: -2.0, y: 0.0, z: 0.0 },
        radius: 1.0,
        material: box mat.clone()
    };
    prims.push(box sphere_top);
    prims.push(box sphere_mid);
    prims.push(box sphere_bot);

    let scene = Scene {
        lights: lights,
        background: Vec3::one(),
        prim_strat: box VecPrimContainer::new(prims),
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
