use std::rand::{task_rng, Rng, SeedableRng, Isaac64Rng};
use std::sync::Arc;
use std::sync::TaskPool;
use std::num::FloatMath;
use raytracer::compositor::{ColorRGBA, Surface, SurfaceFactory};
use raytracer::{Intersection, KDNode, KDTree, Photon, PhotonQuery, Ray};
use std::collections::BinaryHeap;

use light::Light;
use scene::{Camera, Scene};
use vec3::Vec3;

use std::num::Float;
use std::f64::consts::PI;

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
        let photon_cache = Renderer::shoot_photons(photon_scene_local.deref(), 10000, 0.01, 10);

        // let target = BBox {
        //     min: Vec3 {x: -100.0, y:-100.0, z: -100.0},
        //     max: Vec3 {x: 100.0, y: 100.0, z: 100.0}
        // };

        // let test_results = KDNode::query_region(photon_cache, target);
        // panic!("TEST {}", test_results.len());

        let mut surface = Surface::new(camera.image_width as uint,
                                       camera.image_height as uint,
                                       ColorRGBA::new_rgb(0, 0, 0));

        let pool = TaskPool::new(self.tasks);

        let (tx, rx) = channel();

        let mut jobs = 0;

        for subsurface_factory in surface.divide(128, 8) {
            jobs += 1;

            let renderer = *self.clone();
            let child_tx = tx.clone();
            let scene_local = shared_scene.clone();
            let camera_local = camera.clone();
            let photon_cache_local = photon_cache.clone();

            pool.execute(proc() {
                child_tx.send(renderer.render_tile(camera_local.clone(),
                    scene_local.deref(), subsurface_factory, &photon_cache_local.clone()));
            });
        }

        let start_time = ::time::get_time();

        for (i, subsurface) in rx.iter().take(jobs).enumerate() {
            surface.merge(subsurface);
            ::util::print_progress("Tile", start_time, (i + 1) as uint, jobs);
        }
        surface
    }

    fn render_tile(&self, camera: Camera, scene: &Scene,
                   tile_factory: SurfaceFactory, photon_cache: &Box<KDNode>) -> Box<Surface> {

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

                // Supersampling, jitter algorithm
                let pixel_width = 1.0 / pixel_samples as f64;
                let mut color = Vec3::zero();

                for y_subpixel in range(0, pixel_samples) {
                    for x_subpixel in range(0, pixel_samples) {
                        // Don't jitter if not antialiasing
                        let (j_x, j_y) = if pixel_samples > 1 {
                            (x_subpixel as f64 * pixel_width + rng.gen::<f64>() * pixel_width,
                             y_subpixel as f64 * pixel_width + rng.gen::<f64>() * pixel_width)
                        } else {
                            (0.0, 0.0)
                        };

                        let ray = camera.get_ray(abs_x as f64 + j_x, abs_y as f64 + j_y);
                        let result = Renderer::trace(scene, &ray, shadow_samples,
                                                     reflect_depth, refract_depth, false, photon_cache);
                        // Clamp subpixels for now to avoid intense aliasing when combined value is clamped later
                        // Should think of a better way to handle this
                        color = color + result.clamp(0.0, 1.0).scale(1.0 / (pixel_samples * pixel_samples) as f64);
                    }
                }
                tile[(rel_x, rel_y)] = ColorRGBA::new_rgb_clamped(color.x, color.y, color.z);
            }
        }

        box tile
    }

    fn shoot_photons(scene: &Scene, photon_count: uint, power_threshold: f64, max_bounces: uint) -> Box<KDNode> {
        let mut photons: Vec<Photon> = Vec::new();

        let start_time = ::time::get_time();
        let mut count = 0u;
        let total = photon_count * scene.lights.len();

        for light in scene.lights.iter() {
            for _ in range(0, photon_count) {
                let ray = Ray::new(light.position(), Vec3::random());

                photons = photons + Renderer::shoot_photon(scene, &ray, power_threshold,
                                                           light.color(), max_bounces, 0, false);

                count += 1;
                ::util::print_progress("Photon", start_time, count, total);
            }
        }

        match KDTree::new_from_photons(photons, 0) {
            Some(tree) => tree,
            None => panic!("Could not generate photon cache. Photon count is 0/KDTree is empty?")
        }
    }

    fn shoot_photon(scene: &Scene, ray: &Ray, power_threshold: f64,
                    power: Vec3, max_bounces: uint, bounces: uint, inside: bool) -> Vec<Photon> {

        let mut photons: Vec<Photon> = Vec::new();

        if bounces > max_bounces || power.len() < power_threshold {
            return photons;
        }

        match ray.get_nearest_hit(scene) {
            Some(nearest_hit) => {
                let n = nearest_hit.n.unit();
                let i = (-ray.direction).unit();
                let l = Vec3::reflect(&n, &ray.direction);
                let u = nearest_hit.u;
                let v = nearest_hit.v;

                // Randomly decide to reflect/transmit/absorb
                let mut rng = task_rng();
                let rand = rng.gen::<f64>();
                let p_diffuse_reflect = nearest_hit.material.global_diffuse();
                let p_reflect = nearest_hit.material.global_specular();
                let p_transmit = nearest_hit.material.global_transmissive();
                let p_absorb = 1.0 - p_reflect - p_transmit - p_diffuse_reflect;

                if rand < p_absorb {
                    // ---- LINE ---
                    //  END OF LINE
                    photons.push(Photon {
                        position: nearest_hit.position,
                        incoming_dir: ray.direction.scale(-1.0),
                        power: power
                    });
                } else if rand < p_transmit + p_absorb {
                    // Transmit
                    // This if condition is an optimisation hack. For subsurface lighting
                    // we need a proper ray-marching implementation; since that doesn't exist
                    // in here we just kill the ray if the material is not refractive
                    if nearest_hit.material.is_refractive() {
                        let t = match Vec3::refract(&i, &n, nearest_hit.material.ior(), inside) {
                            Some(ref t) => *t,
                            None => Vec3::reflect(&i, &n)
                        };
                        let refract_ray = Ray::new(nearest_hit.position + t.scale(EPSILON), t);
                        let photon_color = power * Vec3::clamp(&(nearest_hit.material.sample(n, i, l, u, v)), 0.0, 1.0);
                        photons = photons + Renderer::shoot_photon(scene, &refract_ray, power_threshold,
                                                                   photon_color, max_bounces, bounces + 1, !inside);
                    }
                } else {
                    // Reflect (always reflect for D* (diffuse-*) light interactions)
                    let r = Vec3::reflect(&i, &n);
                    let reflect_ray = Ray::new(nearest_hit.position, r);

                    let photon_color = power * Vec3::clamp(&(nearest_hit.material.sample(n, i, l, u, v)), 0.0, 1.0);
                    photons = photons + Renderer::shoot_photon(scene, &reflect_ray, power_threshold,
                                                               photon_color, max_bounces, bounces + 1, inside);
                }
            },
            None => {}
        }

        photons
    }

    fn trace(scene: &Scene, ray: &Ray, shadow_samples: uint,
             reflect_depth: uint, refract_depth: uint, inside: bool, photon_cache: &Box<KDNode>) -> Vec3 {

        if reflect_depth <= 0 || refract_depth <= 0 { return Vec3::zero() }

        match ray.get_nearest_hit(scene) {
            Some(hit) => {
                let n = hit.n.unit();
                let i = (-ray.direction).unit();

                // Local lighting computation: surface shading, shadows
                let mut result = scene.lights.iter().fold(Vec3::zero(), |color_acc, light| {
                    let shadow = Renderer::shadow_intensity(scene, &hit, light, shadow_samples);
                    let l = (light.center() - hit.position).unit();

                    color_acc + light.color() * hit.material.sample(n, i, l, hit.u, hit.v) * shadow
                });

                if hit.material.is_reflective() || hit.material.is_refractive() {
                    let reflect_fresnel = Renderer::fresnel_reflect(hit.material.ior(), &i, &n, inside);
                    let mut refract_fresnel = 1.0 - reflect_fresnel;

                    // Global reflection
                    if hit.material.is_reflective() {
                        let r = Vec3::reflect(&i, &n);
                        let reflect_ray = Ray::new(hit.position, r);
                        let reflection = Renderer::trace(scene, &reflect_ray, shadow_samples,
                                                         reflect_depth - 1, refract_depth, inside, photon_cache);

                        result = result + reflection.scale(hit.material.global_specular()).scale(reflect_fresnel);
                    }

                    // Global refraction
                    if hit.material.is_refractive() {
                        let t = match Vec3::refract(&i, &n, hit.material.ior(), inside) {
                            Some(ref t) => *t,
                            None => {
                                refract_fresnel = 1.0; // Total internal reflection (TODO: verify)
                                Vec3::reflect(&i, &n)
                            }
                        };

                        let refract_ray = Ray::new(hit.position + t.scale(EPSILON), t);
                        let refraction = Renderer::trace(scene, &refract_ray, shadow_samples,
                                                         reflect_depth, refract_depth - 1, !inside, photon_cache);

                        result = result + refraction.scale(hit.material.global_transmissive()).scale(refract_fresnel);
                    }
                }

                // Add indirect illumination estimate
                let initial_max_dist = 150.0; // This was winged. TODO: make this a configurable setting
                let max_photons = 10; // This was winged as well. TODO: make this a configurable setting
                let mut nearby_photons: BinaryHeap<PhotonQuery> = BinaryHeap::with_capacity(max_photons + 1);
                KDNode::query_nearest(&mut nearby_photons, photon_cache.clone(), hit.position, initial_max_dist, max_photons);

                let mut photons = Vec::new();
                for p in nearby_photons.iter() {
                    photons.push(p.photon)
                }

                let flux_sum = photons.iter().fold(Vec3::zero(), |flux_acc, p| {
                    flux_acc + hit.material.brdf(n, p.incoming_dir, i, hit.u, hit.v).clamp(0.0, 1.0) * p.power
                });

                let indirect_irradiance = match nearby_photons.top() {
                    Some(photon_query) => {
                        let photon_spread = photon_query.distance_to_point.abs();
                        flux_sum.scale(1.0 / (2.0 * PI * photon_spread))
                    },
                    None => Vec3::zero()
                };

                indirect_irradiance
                // result + indirect_irradiance
            },
            None => {
                match scene.skybox {
                    Some(ref skybox) => skybox.color(ray.direction),
                    None => scene.background
                }
            }
        }
    }

    fn shadow_intensity(scene: &Scene, hit: &Intersection,
                        light: &Box<Light+Send+Sync>, shadow_samples: uint) -> Vec3 {

        if shadow_samples <= 0 { return Vec3::one() }

        // Point light speedup (no point in sampling a point light multiple times)
        let shadow_sample_tries = if light.is_point() { 1 } else { shadow_samples };
        let mut shadow = Vec3::zero();

        // Take average shadow color after jittering/sampling light position
        for _ in range(0, shadow_sample_tries) {
            // L has to be a unit vector for t_max 1:1 correspondence to
            // distance to light to work. Shadow feelers only search up
            // until light source.
            let sampled_light_position = light.position();
            let shadow_l = (sampled_light_position - hit.position).unit();
            let shadow_ray = Ray::new(hit.position, shadow_l);
            let distance_to_light = (sampled_light_position - hit.position).len();

            // Check against candidate primitives in scene for occlusion
            // and multiply shadow color by occluders' shadow colors
            let candidate_nodes = scene.octree.get_intersected_objects(&shadow_ray);

            shadow = shadow + candidate_nodes.fold(Vec3::one(), |shadow_acc, prim| {
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
    /// http://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
    fn fresnel_reflect(ior: f64, i: &Vec3, n: &Vec3, inside: bool) -> f64 {
        let (n1, n2) = if inside { (ior, 1.0) } else { (1.0, ior) };
        let actual_n = if inside { -*n } else { *n };

        let r0_sqrt = (n1 - n2) / (n1 + n2);
        let r0 = r0_sqrt * r0_sqrt;

        let cos_angle = if n1 <= n2 {
            i.dot(&actual_n)
        } else {
            let t = match Vec3::refract(i, &-actual_n, ior, inside) {
                Some(x) => x,
                None => return 1.0 // n1 > n2 && TIR
            };

            -actual_n.dot(&t) // n1 > n2 && !TIR
        };

        let cos_term = 1.0 - cos_angle;

        (r0 + ((1.0 - r0) * cos_term * cos_term * cos_term * cos_term * cos_term)).max(0.0).min(1.0)
    }
}
