use vec3::Vec3;
use scene::Scene;
use intersection::Intersection;
use std::f64::INFINITY;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn get_nearest_hit<'a>(&self, scene: &'a Scene) -> Option<Intersection<'a>> {
        // TODO: replace scene with candidate objects
        let t_min = 0.001;
        let mut nearest_t = INFINITY;
        let mut nearest_hit = None;

        for prim in scene.prims.iter() {
            let intersection = prim.intersects(self, t_min, nearest_t);

            match intersection {
                Some(intersection) => {
                    if nearest_t == INFINITY || (intersection.t > t_min && intersection.t < nearest_t) {
                        nearest_hit = Some(intersection);
                        nearest_t = intersection.t;
                    }
                    // match nearest_hit {
                    //     Some(nearest_hit) => {
                    //         if intersection.t > t_min && intersection.t < nearest_t {
                    //             nearest_hit = intersection;
                    //             nearest_t = nearest_hit.t;
                    //         }
                    //     }
                    //     None => {}
                    // }
                }
                None => {}
            }
        }

        nearest_hit
    }
}
