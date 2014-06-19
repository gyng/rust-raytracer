use std::rand::{task_rng, Rng, SeedableRng, Isaac64Rng};
use std::sync::Arc;
use std::sync::deque::{BufferPool, Data, Empty, Abort};

use raytracer::compositor::{
    Surface,
    SurfaceFactory,
    ColorRGBA
};
use raytracer::Ray;
use scene::{Camera, Scene};
use vec3::Vec3;


pub struct Renderer {
    pub reflect_depth: int,  // Maximum reflection recursions.
    pub refract_depth: int,  // Maximum refraction recursions. A sphere takes up 2 recursions.
    pub shadow_samples: int, // Number of samples for soft shadows and area lights.
    pub pixel_samples: int,  // The square of this is the number of samples per pixel.
    pub tasks: int           // Minimum number of tasks to spawn.
}


impl Renderer {
    pub fn render(&self, camera: Camera, scene: Scene) -> Surface {

        let mut surface = Surface::new(camera.image_width as uint,
                                   camera.image_height as uint,
                                   ColorRGBA::new_rgb(0, 0, 0));

        let shared_scene = Arc::new(scene);
        let (worker, stealer) = BufferPool::new().deque();
        let (tx, rx) = channel();  // Responses

        let mut jobs = 0;
        for subsurface_factory in surface.divide(128, 8) {
            jobs += 1;
            worker.push(subsurface_factory);
        }

        for _ in range(0, self.tasks) {
            let renderer = *self.clone();
            let child_tx = tx.clone();
            let child_stealer = stealer.clone();
            let scene_local = shared_scene.clone();

            spawn(proc() {
                loop {
                    match child_stealer.steal() {
                        Data(factory) => {
                            child_tx.send(renderer.render_tile(camera,
                                                               scene_local.deref(),
                                                               factory))
                        },
                        Empty => break,
                        Abort => (),
                    }
                }
            });
        }

        let start_time = ::time::get_time().sec;

        for i in range(0, jobs) {
            surface.merge(rx.recv());
            let progress: f64 = 100f64 * (i + 1) as f64 / jobs as f64;
            let current_time = ::time::get_time().sec;
            let remaining_time = (current_time - start_time) as f64 / (i+1) as f64 * (jobs - (i+1)) as f64 / 60.0;
            print!("\rTile {}/{} obtained\tETA {} minutes \t{}%           ",
                   (i + 1), jobs, ::std::f64::to_str_exact(remaining_time, 2), ::std::f64::to_str_exact(progress, 2));
            ::std::io::stdio::flush();
        }
        println!("");

        surface
    }

    fn render_tile(&self, camera: Camera, scene: &Scene,
                   tile_factory: SurfaceFactory) -> Box<Surface> {

        let shadow_samples = self.shadow_samples;
        let pixel_samples = self.pixel_samples;
        let reflect_depth = self.reflect_depth;
        let refract_depth = self.refract_depth;

        let mut tile = tile_factory.create();

        let mut random_data = [0u64, ..64];
        for i in range(0u, 64u) {
            random_data[i] = task_rng().next_u64();
        }
        let mut rng: Isaac64Rng = SeedableRng::from_seed(random_data.clone());

        for rel_y in range(0u, tile.height) {
            let abs_y = (camera.image_height as uint) - (tile.y_off + rel_y) - 1;
            for rel_x in range(0u, tile.width) {
                let abs_x = tile.x_off + rel_x;

                let mut color = Vec3::zero();

                // Supersampling, jitter algorithm
                let pixel_width = 1.0 / pixel_samples as f64;

                for y_subpixel in range(0, pixel_samples) {
                    for x_subpixel in range(0, pixel_samples) {
                        let mut j_x = abs_x as f64;
                        let mut j_y = abs_y as f64;

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
                *tile.get_mut(rel_x, rel_y) =
                    ColorRGBA::new_rgb_clamped(color.x, color.y, color.z);
            }
        }

        box tile
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
                            // TODO: Clean up
                            match scene.octree {
                                Some(ref octree) => {
                                    let candidate_nodes = octree.get_intersection_objects(&shadow_ray);
                                    let mut sample_shadow = Vec3::one();

                                    for node in candidate_nodes.iter() {
                                        let prim = scene.prims.get(node.index);
                                        let occlusion = prim.intersects(&shadow_ray, epsilon, distance_to_light);

                                        match occlusion {
                                            Some(occlusion) => {
                                                sample_shadow = sample_shadow * occlusion.material.transmission()
                                            }
                                            None => {}
                                        }
                                    }

                                    shadow = shadow + sample_shadow;
                                }
                                None => {
                                    shadow = shadow + scene.prims.iter().fold(Vec3::one(), |shadow_acc, prim| {
                                        let occlusion = prim.intersects(&shadow_ray, epsilon, distance_to_light);
                                        match occlusion {
                                            Some(occlusion) => {shadow_acc * occlusion.material.transmission()}
                                            None => shadow_acc
                                        }
                                    });
                                }
                            }
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
