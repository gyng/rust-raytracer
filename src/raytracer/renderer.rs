use light::Light;
use raytracer::compositor::{ColorRGBA, Surface, SurfaceFactory};
use raytracer::{Intersection, Ray};
use scene::{Camera, Scene};
use std::num::Float;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::mpsc::channel;
use vec3::Vec3;
use rand::{thread_rng, Rng, Isaac64Rng};
use threadpool::ThreadPool;

pub static EPSILON: f64 = ::std::f64::EPSILON * 10000.0;

#[derive(Clone)]
pub struct Renderer {
    pub reflect_depth: u32,  // Maximum reflection recursions.
    pub refract_depth: u32,  // Maximum refraction recursions. A sphere takes up 2 recursions.
    pub shadow_samples: u32, // Number of samples for soft shadows and area lights.
    pub pixel_samples: u32,  // The square of this is the number of samples per pixel.
    pub tasks: usize         // Minimum number of tasks to spawn.
}

impl Renderer {
    pub fn render(&self, camera: Camera, shared_scene: Arc<Scene>) -> Surface {

        let mut surface = Surface::new(camera.image_width as usize,
                                       camera.image_height as usize,
                                       ColorRGBA::new_rgb(0, 0, 0));

        let pool = ThreadPool::new(self.tasks);

        let (tx, rx) = channel();

        let mut jobs = 0;

        for subsurface_factory in surface.divide(128, 8) {
            jobs += 1;

            let renderer = self.clone();
            let child_tx = tx.clone();
            let scene_local = shared_scene.clone();
            let camera_local = camera.clone();

            pool.execute(move || {
                let _ = child_tx.send(renderer.render_tile(camera_local.clone(),
                    scene_local.deref(), subsurface_factory));
            });
        }
        drop(tx);

        let start_time = ::time::get_time();

        for (i, subsurface) in rx.iter().enumerate() {
            surface.merge(&subsurface);
            ::util::print_progress("Tile", start_time.clone(), (i + 1) as usize, jobs);
        }
        surface
    }

    fn render_tile(&self, camera: Camera, scene: &Scene,
                   tile_factory: SurfaceFactory) -> Surface {

        let shadow_samples = self.shadow_samples;
        let pixel_samples = self.pixel_samples;
        let reflect_depth = self.reflect_depth;
        let refract_depth = self.refract_depth;

        let mut tile = tile_factory.create();

        let mut rng: Isaac64Rng = thread_rng().gen();

        for rel_y in 0usize..tile.height {
            let abs_y = camera.image_height as usize - (tile.y_off + rel_y) - 1;
            for rel_x in 0usize..tile.width {
                let abs_x = tile.x_off + rel_x;

                // Supersampling, jitter algorithm
                let pixel_width = 1.0 / pixel_samples as f64;
                let mut color = Vec3::zero();

                for y_subpixel in 0u32..pixel_samples {
                    for x_subpixel in 0u32..pixel_samples {
                        // Don't jitter if not antialiasing
                        let (j_x, j_y) = if pixel_samples > 1 {
                            (x_subpixel as f64 * pixel_width + rng.gen::<f64>() * pixel_width,
                             y_subpixel as f64 * pixel_width + rng.gen::<f64>() * pixel_width)
                        } else {
                            (0.0, 0.0)
                        };

                        let ray = camera.get_ray(abs_x as f64 + j_x, abs_y as f64 + j_y);
                        let result = Renderer::trace(scene, &ray, shadow_samples,
                                                     reflect_depth, refract_depth, false);
                        // Clamp subpixels for now to avoid intense aliasing when combined value is clamped later
                        // Should think of a better way to handle this
                        color = color + result.clamp(0.0, 1.0).scale(1.0 / (pixel_samples * pixel_samples) as f64);
                    }
                }
                tile[(rel_x, rel_y)] = ColorRGBA::new_rgb_clamped(color.x, color.y, color.z);
            }
        }

        tile
    }

    fn trace(scene: &Scene, ray: &Ray, shadow_samples: u32,
             reflect_depth: u32, refract_depth: u32, inside: bool) -> Vec3 {

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
                                                         reflect_depth - 1, refract_depth, inside);

                        result = result + hit.material.global_specular(&reflection).scale(reflect_fresnel);
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
                                                         reflect_depth, refract_depth - 1, !inside);

                        result = result + hit.material.global_transmissive(&refraction).scale(refract_fresnel);
                    }
                }

                result
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
                        light: &Box<Light+Send+Sync>, shadow_samples: u32) -> Vec3 {

        if shadow_samples <= 0 { return Vec3::one() }

        // Point light speedup (no point in sampling a point light multiple times)
        let shadow_sample_tries = if light.is_point() { 1 } else { shadow_samples };
        let mut shadow = Vec3::zero();

        // Take average shadow color after jittering/sampling light position
        for _ in 0..shadow_sample_tries {
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

#[test]
fn it_renders_the_background_of_an_empty_scene() {
    let camera = Camera::new(
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        45.0,
        32,
        32
    );

    let test_scene = Scene {
        lights: vec!(),
        octree: ::raytracer::Octree::new_from_prims(vec!()),
        background: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        skybox: None
    };

    let shared_scene = Arc::new(test_scene);

    let renderer = Renderer {
        reflect_depth: 1,
        refract_depth: 1,
        shadow_samples: 1,
        pixel_samples: 1,
        tasks: 2
    };

    let image_data = renderer.render(camera, shared_scene);

    for color in image_data.buffer.iter() {
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }
}
