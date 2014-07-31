use std::rand::{task_rng, Rng, SeedableRng, Isaac64Rng};
use std::sync::Arc;
use std::sync::deque::{BufferPool, Data, Empty, Abort};

use geometry::bbox;
use raytracer::compositor::{
    Surface,
    SurfaceFactory,
    ColorRGBA
};
use raytracer::{Intersection, KDNode, KDTree, Photon, Ray};
use light::Light;
use scene::{Camera, Scene};
use vec3::Vec3;


pub static EPSILON: f64 = ::std::f64::EPSILON * 10000.0;


pub struct Renderer {
    pub reflect_depth: uint,  // Maximum reflection recursions.
    pub refract_depth: uint,  // Maximum refraction recursions. A sphere takes up 2 recursions.
    pub shadow_samples: uint, // Number of samples for soft shadows and area lights.
    pub pixel_samples: uint,  // The square of this is the number of samples per pixel.
    pub tasks: uint           // Minimum number of tasks to spawn.
}


impl Renderer {
    pub fn render(&self, camera: Camera, shared_scene: Arc<Scene>) -> Surface {

        let photon_scene_local = shared_scene.clone();
        let photon_cache = Renderer::shoot_photons(photon_scene_local.deref(), 128000, 0.01, 100);

        let mut surface = Surface::new(camera.image_width as uint,
                                       camera.image_height as uint,
                                       ColorRGBA::new_rgb(0, 0, 0));

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
            let camera_local = camera.clone();
            let photon_cache_local = photon_cache.clone();

            spawn(proc() {
                loop {
                    match child_stealer.steal() {
                        Data(factory) => {
                            child_tx.send(renderer.render_tile(camera_local.clone(),
                                                               scene_local.deref(),
                                                               factory,
                                                               &photon_cache_local.clone()))
                        },
                        Empty => break,
                        Abort => ()
                    }
                }
            });
        }

        let start_time = ::time::get_time();

        for i in range(0, jobs) {
            surface.merge(rx.recv());
            ::util::print_progress("Tile", start_time, (i + 1) as uint, jobs);
        }

        surface
    }

    fn render_tile(&self, camera: Camera, scene: &Scene,
                   tile_factory: SurfaceFactory, photon_cache: &KDNode) -> Box<Surface> {

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
                        let result = Renderer::trace(scene, &ray, shadow_samples, reflect_depth, refract_depth, false, photon_cache);
                        color = color + result.scale(1.0 / (pixel_samples * pixel_samples) as f64);
                    }
                }
                *tile.get_mut(rel_x, rel_y) = ColorRGBA::new_rgb_clamped(color.x, color.y, color.z);
            }
        }

        box tile
    }

    fn shoot_photons(scene: &Scene, photon_count: uint, power_threshold: f64, max_bounces: uint) -> KDNode {
        let mut photons: Vec<Photon> = Vec::new();

        let start_time = ::time::get_time();
        let mut count = 0u;
        let total = photon_count * scene.lights.len();

        for light in scene.lights.iter() {
            for _ in range(0, photon_count) {
                let ray = Ray {
                    origin: light.position(),
                    direction: Vec3::random()
                };

                // TODO: light color should replace power argument?
                photons.push_all_move(Renderer::shoot_photon(scene, &ray, power_threshold,
                                                             light.color(), max_bounces, 0));

                count += 1;
                ::util::print_progress("Photon", start_time, count, total);
            }
        }

        match KDTree::new_from_photons(photons, 0) {
            Some(tree) => tree,
            None => fail!("Could not generate photon cache. Photon count is 0/KDTree is empty?")
        }
    }

    fn shoot_photon(scene: &Scene, ray: &Ray, power_threshold: f64,
                    power: Vec3, max_bounces: uint, bounces: uint) -> Vec<Photon> {

        let mut photons: Vec<Photon> = Vec::new();

        if bounces > max_bounces || power.len() < power_threshold {
            return photons;
        }

        match ray.get_nearest_hit(scene) {
            Some(nearest_hit) => {
                // We want caustics only, so photons which don't hit a specular material can be killed
                if bounces == 0 && !nearest_hit.material.is_reflective() && !nearest_hit.material.is_refractive() {
                    return photons;
                }

                let n = nearest_hit.n.unit();
                let i = ray.direction.scale(-1.0).unit();
                // let l = Vec3::random();
                let u = nearest_hit.u;
                let v = nearest_hit.v;

                // Randomly decide to reflect/transmit/absorb, Russian roulette
                // In real life we split the photon up, but it's too slow as each photon
                // intersection generates 3 new photons (absorb/reflect/refract) that way
                // let mut rng = task_rng();
                // let rand = rng.gen::<f64>();
                // let p_reflect  = nearest_hit.material.global_specular(&Vec3::one()).len() / 2.0_f64.sqrt();
                // let p_transmit = nearest_hit.material.global_transmissive(&Vec3::one()).len() / 2.0_f64.sqrt();
                // let p_absorb = 1.0 - p_reflect - p_transmit;

                // if rand < p_reflect {
                    // Reflected
                    if nearest_hit.material.is_reflective() {
                        let r = Vec3::reflect(&i, &n);
                        let reflect_ray = Ray {origin: nearest_hit.position, direction: r};

                        let reflect_scaled = nearest_hit.material.global_specular(&power);
                        photons = photons + Renderer::shoot_photon(scene, &reflect_ray, power_threshold,
                                                                   reflect_scaled, max_bounces, bounces + 1);
                    }
                // } else if rand < p_reflect + p_transmit {
                    // Transmitted (refracted)

                    if nearest_hit.material.is_refractive() {
                        let t = match Vec3::refract(&i, &n, nearest_hit.material.ior(), false) {
                            Some(ref t) => *t,
                            None => Vec3::reflect(&i, &n)
                        };
                        let refract_ray = Ray {origin: nearest_hit.position + t.scale(EPSILON), direction: t};
                        let refract_scaled = nearest_hit.material.global_transmissive(&power);
                        // let photon_color = Vec3::clamp(&(nearest_hit.material.sample(n, i, l, u, v)));
                        photons = photons + Renderer::shoot_photon(scene, &refract_ray, power_threshold,
                                                                   refract_scaled, max_bounces, bounces + 1);
                    }
                // } else {
                    // Absorbed
                    photons.push(Photon {
                        position: nearest_hit.position,
                        incoming_dir: ray.direction.scale(-1.0),
                        power: power
                    });
                // }

                // Approach 2: slow

                // Generate three photons: absorb/reflect/refract
                // Store incoming indirect lighting
                // photons.push(Photon {
                //     position: nearest_hit.position,
                //     incoming_dir: ray.direction.scale(-1.0),
                //     power: power
                // });

                // // Send out indirect diffuse lighting
                // let photon_color = Vec3::clamp(&(nearest_hit.material.sample(n, i, i, u, v)));
                // let r = Vec3::reflect(&i, &n);
                // let reflect_ray = Ray {origin: nearest_hit.position, direction: r};
                // photons.push_all_move(Renderer::shoot_photon(scene, &reflect_ray, power_threshold,
                //                                              photon_color, max_bounces, bounces + 1));
            },
            None => {}
        }

        photons
    }

    fn trace(scene: &Scene, ray: &Ray, shadow_samples: uint,
             reflect_depth: uint, refract_depth: uint, inside: bool, photon_cache: &KDNode) -> Vec3 {

        if reflect_depth <= 0 || refract_depth <= 0 { return Vec3::zero() }

        match ray.get_nearest_hit(scene) {
            Some(nearest_hit) => {
                let n = nearest_hit.n.unit();
                let i = (ray.direction.scale(-1.0)).unit();

                // Local lighting computation: surface shading, shadows
                let mut result = scene.lights.iter().fold(Vec3::zero(), |color_acc, light| {
                    let shadow = Renderer::shadow_intensity(scene, &nearest_hit, light, shadow_samples);
                    let l = (light.center() - nearest_hit.position).unit();
                    let u = nearest_hit.u;
                    let v = nearest_hit.v;

                    color_acc + light.color() * nearest_hit.material.sample(n, i, l, u, v) * shadow
                });

                if nearest_hit.material.is_reflective() ||
                   nearest_hit.material.is_refractive() {

                    let cos_angle = -ray.direction.dot(&n);
                    let reflect_fresnel = Renderer::fresnel_reflect(nearest_hit.material.ior(), cos_angle);
                    let mut refract_fresnel = 1.0 - reflect_fresnel;

                    // Global reflection
                    if nearest_hit.material.is_reflective() {
                        let r = Vec3::reflect(&i, &n);
                        let reflect_ray = Ray {origin: nearest_hit.position, direction: r};
                        let reflection = Renderer::trace(scene, &reflect_ray, shadow_samples,
                                                         reflect_depth - 1, refract_depth, inside, photon_cache);

                        result = result + nearest_hit.material.global_specular(&reflection).scale(reflect_fresnel);
                    }

                    // Global refraction
                    if nearest_hit.material.is_refractive() {
                        let t = match Vec3::refract(&i, &n, nearest_hit.material.ior(), inside) {
                            Some(ref t) => *t,
                            None => {
                                refract_fresnel = 1.0; // Total internal reflection (TODO: check that this is working)
                                Vec3::reflect(&i, &n)
                            }
                        };

                        let refract_ray = Ray {origin: nearest_hit.position + t.scale(EPSILON), direction: t};
                        let refraction = Renderer::trace(scene, &refract_ray, shadow_samples,
                                                         reflect_depth, refract_depth - 1, !inside, photon_cache);

                        result = result + nearest_hit.material.global_transmissive(&refraction).scale(refract_fresnel);
                    }
                }

                // Get photon cache result for caustics/indirect lighting
                // TODO: Use n-nearest photons instead of querying a region
                let search_half_width = 3.0;
                let target = bbox::union_points(&nearest_hit.position, &nearest_hit.position).expand(search_half_width);
                let photons = KDNode::query_region(photon_cache, target);
                let mut power = Vec3::zero();
                for photon in photons.iter() {
                    power = power + Vec3::clamp(&photon.power)
                }
                // This irradiance is completely wrong
                // let caustics = power.scale(1.0 / (photons.len() as f64 + 1.0));
                let total_photons = 128000.0;
                let caustics = power.scale(1.0 / (search_half_width * search_half_width * 6.0 * 2.0 * total_photons / 32000.0));

                // let indirect_irradiance = match KDNode::nearest_neighbour(&Some(box photon_cache.clone()), nearest_hit.position, None) {
                //     Some(p) => p.power,
                //     None => Vec3::zero()
                // };


                result = result + caustics;
                // result = indirect_irradiance;

                result
            }

            None => {
                match scene.skybox {
                    Some(ref skybox) => skybox.color(ray.direction),
                    None => scene.background
                }
            }
        }
    }

    fn shadow_intensity(scene: &Scene, nearest_hit: &Intersection,
                        light: &Box<Light+Send+Share>, shadow_samples: uint) -> Vec3 {

        if shadow_samples <= 0 { return Vec3::one(); }

        // Point light speedup (no point in sampling a point light multiple times)
        let shadow_sample_tries = if light.is_point() { 1 } else { shadow_samples };
        let mut shadow = Vec3::zero();

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

            let candidate_nodes = scene.prim_strat.get_intersection_objects(&shadow_ray);

            shadow = shadow + candidate_nodes.iter().fold(Vec3::one(), |shadow_acc, prim| {
                let occlusion = prim.intersects(&shadow_ray, EPSILON, distance_to_light);
                match occlusion {
                    Some(occlusion) => shadow_acc * occlusion.material.transmission(),
                    None => shadow_acc
                }
            });
        }

        shadow.scale(1.0 / shadow_sample_tries as f64)
    }

    /// Calculates the fresnel (reflectivity) given the index of refraction and the cos_angle
    /// This uses Schlick's approximation. cos_angle is normal_dot_incoming
    fn fresnel_reflect(ior: f64, cos_angle: f64) -> f64 {
        let n1 = 1.0;
        let n2 = ior;
        let r0 = ((n1 - n2) / (n1 + n2)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cos_angle).powf(5.0)
    }
}
