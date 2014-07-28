use std::f64::INFINITY;
use raytracer::Intersection;
use scene::Scene;
use vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
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
                }
                None => {nearest_hit}
            };
        }

        nearest_hit
    }
}
