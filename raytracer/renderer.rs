use std::rand::{task_rng, Rng, SeedableRng, Isaac64Rng};
use std::sync::Arc;
use raytracer::compositor::composite;
use raytracer::{Ray, Tile};
use scene::{Camera, Scene};
use vec3::Vec3;

pub struct Renderer {
    pub reflect_depth: int,  // Maximum reflection recursions.
    pub refract_depth: int,  // Maximum refraction recursions. A sphere takes up 2 recursions.
    pub use_octree: bool,    // Unimplemented. Use octree/k-d tree?
    pub shadow_samples: int, // Number of samples for soft shadows and area lights.
    pub pixel_samples: int,  // The square of this is the number of samples per pixel.
    pub tasks: int           // Minimum number of tasks to spawn.
}

impl Renderer {
    pub fn render(&self, camera: Camera, scene: Scene) -> Vec<Vec3> {
        let mut tiles = Vec::with_capacity(self.tasks as uint);
        let tiles_per_side = (self.tasks as f64).sqrt().ceil() as int;
        let tile_width  = (camera.image_width  as f64 / tiles_per_side as f64) as int;
        let tile_height = (camera.image_height as f64 / tiles_per_side as f64) as int;

        let shared_scene = Arc::new(scene);
        let (tx, rx) = channel();

        for tile_x in range(0, tiles_per_side) {
            for tile_y in range(0, tiles_per_side) {
                let scene_local = shared_scene.clone();
                let child_tx = tx.clone();

                let shadow_samples_local = self.shadow_samples;
                let pixel_samples_local = self.pixel_samples;
                let reflect_depth_local = self.reflect_depth;
                let refract_depth_local = self.refract_depth;

                let start_x = tile_x * tile_width;
                let start_y = tile_y * tile_height;
                let end_x = start_x + tile_width;
                let end_y = start_y + tile_height;

                spawn(proc() {
                    child_tx.send(Renderer::render_tile(camera,
                                                        scene_local.deref(),
                                                        shadow_samples_local,
                                                        pixel_samples_local,
                                                        reflect_depth_local,
                                                        refract_depth_local,
                                                        start_x, start_y,
                                                        end_x, end_y));
                });

            }
        }

        for _ in range(0, tiles_per_side * tiles_per_side) {
            tiles.push(rx.recv())
        }

        composite(tiles, camera.image_width, camera.image_height)
    }

    fn render_tile(camera: Camera,
                   scene: &Scene,
                   shadow_samples: int,
                   pixel_samples: int,
                   reflect_depth: int,
                   refract_depth: int,
                   from_x: int,
                   from_y: int,
                   to_x: int,
                   to_y: int)
                   -> Tile {
        let width  = to_x - from_x;
        let height = to_y - from_y;
        let tile_size = width * height;
        let mut image_data: Vec<Vec3> = Vec::with_capacity(tile_size as uint);

		let mut random_data = [0u64, ..64];
		for i in range(0u, 64u) {
			random_data[i] = task_rng().next_u64();
		}
		let mut rng: Isaac64Rng = SeedableRng::from_seed(random_data.clone());

        for y in range(from_y, to_y) {
            for x in range(from_x, to_x) {
                let mut color = Vec3::zero();

                // Supersampling, jitter algorithm
                let pixel_width = 1.0 / pixel_samples as f64;

                for y_subpixel in range(0, pixel_samples) {
                    for x_subpixel in range(0, pixel_samples) {
                        let mut j_x = x as f64;
                        let mut j_y = y as f64;

                        // Don't jitter if not antialiasing
                        if pixel_samples > 1 {
                            j_x = j_x + x_subpixel as f64 * pixel_width + rng.gen::<f64>() * pixel_width as f64;
                            j_y = j_y + y_subpixel as f64 * pixel_width + rng.gen::<f64>() * pixel_width as f64;
                        }

                        let ray = camera.get_ray(j_x, j_y);
                        let result = Renderer::trace(scene, &ray, shadow_samples, reflect_depth, refract_depth, false);
                        color = color + result.scale(1.0 / (pixel_samples * pixel_samples) as f64);
                    }
                }

                image_data.push(Vec3 {
                    x: color.x,
                    y: color.y,
                    z: color.z
                });
            }
        }

        Tile {
            from_x: from_x,
            from_y: from_y,
            to_x: to_x,
            to_y: to_y,
            data: image_data
        }
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
