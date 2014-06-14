use camera::Camera;
use scene::Scene;
use ray::Ray;
use vec3::Vec3;
use std::rand::{task_rng, Rng};
// use std::sync::Arc;

pub struct Renderer {
    pub reflect_depth: int,  // Maximum reflection recursions.
    pub refract_depth: int,  // Maximum refraction recursions. A sphere takes up 2 recursions.
    pub use_octree: bool,    // Unimplemented. Use octree/k-d tree?
    pub shadow_samples: int, // Number of samples for soft shadows and area lights.
    pub pixel_samples: int,  // The square of this is the number of samples per pixel.
    pub threads: int         // Unimplemented. Number of threads to use.
}

impl Renderer {
    pub fn render(&self, camera: Camera, scene: Scene) -> Vec<int> {
        // // ABANDONED THREADING SUPPORT
        // let scene_arc = Arc::new(scene);
        // let (tx, rx) = channel();
        // let (arc_tx, arc_rx) = channel();
        // arc_tx.send(scene_arc);

        // // for thread_no in range(0, 1) {
        //     let child_tx = tx.clone();

        //     spawn(proc() {
        //         let local_arc = arc_rx.recv();
        //         let local_scene = &*local_arc;
        //         let result = Renderer::render_tile(camera,
        //                                            local_scene,
        //                                            self.shadow_samples,
        //                                            self.reflect_depth,
        //                                            self.refract_depth,
        //                                            0, 0,
        //                                            camera.image_width, camera.image_height);
        //         child_tx.send(result);
        //     });
        // // }

        // rx.recv()

        Renderer::render_tile(camera,
                              scene,
                              self.shadow_samples,
                              self.pixel_samples,
                              self.reflect_depth,
                              self.refract_depth,
                              0, 0,
                              camera.image_width, camera.image_height)
    }

    fn render_tile(camera: Camera,
                   scene: Scene,
                   shadow_samples: int,
                   pixel_samples: int,
                   reflect_depth: int,
                   refract_depth: int,
                   from_x: int,
                   from_y: int,
                   to_x: int,
                   to_y: int)
                   -> Vec<int> {
        let width  = to_x - from_x;
        let height = to_y - from_y;
        let tile_size = width * height * 3;
        let mut tile: Vec<int> = Vec::with_capacity(tile_size as uint);

        for y in range(from_y, to_y) {
            let inv_y = to_y - y;
            for x in range(from_x, to_x) {
                let mut color = Vec3::zero();

                // Supersampling, jitter algorithm
                let mut rng = task_rng();
                let pixel_width = 1.0 / pixel_samples as f64;

                for _ in range(0, pixel_samples) {
                    for _ in range(0, pixel_samples) {
                        let mut j_x = x as f64 + rng.gen::<f64>() * pixel_width as f64;
                        let mut j_y = inv_y as f64 + rng.gen::<f64>() * pixel_width as f64;

                        // Unwanted jitter if not anti-aliasing
                        if pixel_samples == 1 {
                            j_x = x as f64;
                            j_y = inv_y as f64;
                        }

                        let ray = camera.get_ray(j_x, j_y);
                        let result = Renderer::trace(&scene, &ray, shadow_samples, reflect_depth, refract_depth, false);
                        color = color + result.scale(1.0 / (pixel_samples * pixel_samples) as f64);
                    }
                }

                tile.push((color.x * 255.0) as int);
                tile.push((color.y * 255.0) as int);
                tile.push((color.z * 255.0) as int);
            }
        }

        tile
    }

    fn trace(scene: &Scene,
             ray: &Ray,
             shadow_samples: int,
             reflect_depth: int,
             refract_depth: int,
             inside: bool)
             -> Vec3 {
        if reflect_depth <= 0 || refract_depth <= 0 { return Vec3::zero() }
        let epsilon = ::std::f64::EPSILON * 10000.0;

        match ray.get_nearest_hit(scene) {
            Some(nearest_hit) => {
                let n = nearest_hit.n.unit();
                let i = (ray.direction.scale(-1.0)).unit();

                // Local lighting computation: surface shading, shadows
                let mut result = scene.lights.iter().fold(Vec3::zero(), |color_acc, light| {
                    let mut shadow = Vec3::zero();

                    if shadow_samples > 0 {
                        // Point light speedup
                        let shadow_sample_tries = if light.is_point() { 1 } else { shadow_samples };

                        // Take average shadow color after jittering/sampling light position
                        for _ in range(0, shadow_sample_tries) {
                            // L has to be a unit vector for t_max 1:1 correspondence to
                            // distance to light to work. Shadow feelers only search up
                            // until light source.
                            let sampled_light_position = light.position();
                            let shadow_l = (sampled_light_position - nearest_hit.position).unit();
                            let shadow_ray = Ray {origin: nearest_hit.position, direction: shadow_l};
                            let distance_to_light = (sampled_light_position - nearest_hit.position).len();

                            // Check against candidate primitives in scene for occlusion
                            // and multiply shadow color by occluders' shadow colors
                            shadow = shadow + scene.prims.iter().fold(Vec3::one(), |shadow_acc, prim| {
                                let occlusion = prim.intersects(&shadow_ray, epsilon, distance_to_light);
                                match occlusion {
                                    Some(occulusion) => {shadow_acc * occulusion.material.transmission()}
                                    None => shadow_acc
                                }
                            });
                        }

                        shadow = shadow.scale(1.0 / shadow_sample_tries as f64);
                    } else {
                        shadow = Vec3::one();
                    }

                    let l = (light.center() - nearest_hit.position).unit();

                    color_acc + light.color() * nearest_hit.material.sample(n, i, l) * shadow
                });

                // Global reflection
                if nearest_hit.material.is_reflective() {
                    let r = Vec3::reflect(&i, &n);
                    let reflect_ray = Ray {origin: nearest_hit.position, direction: r};
                    let reflection = Renderer::trace(scene, &reflect_ray, shadow_samples, reflect_depth - 1, refract_depth, inside);

                    result = result + nearest_hit.material.global_specular(&reflection);
                }

                // Global refraction
                if nearest_hit.material.is_refractive() {
                    let t = Vec3::refract(&i, &n, nearest_hit.material.ior(), inside);
                    let refract_ray = Ray {origin: nearest_hit.position + t.scale(epsilon), direction: t};
                    let refraction = Renderer::trace(scene, &refract_ray, shadow_samples, reflect_depth, refract_depth, !inside);

                    result = result + nearest_hit.material.global_transmissive(&refraction);
                }

                result
            }

            None => {scene.background}
        }
    }
}
