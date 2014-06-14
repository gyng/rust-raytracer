use camera::Camera;
use scene::Scene;
use ray::Ray;
use vec3::Vec3;
// use std::sync::Arc;

pub struct Renderer {
    pub reflect_depth: int,
    pub refract_depth: int,
    pub use_octree: bool,
    pub shadows: bool,
    pub threads: int
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
        //                                            self.shadows,
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
                              self.shadows,
                              self.reflect_depth,
                              self.refract_depth,
                              0, 0,
                              camera.image_width, camera.image_height)
    }

    fn render_tile(camera: Camera,
                   scene: Scene,
                   shadows: bool,
                   reflect_depth: int,
                   refract_depth: int,
                   from_x: int,
                   from_y: int,
                   to_x: int,
                   to_y: int)
                   -> Vec<int> {
        // TODO: replace int with uint or better
        let width  = to_x - from_x;
        let height = to_y - from_y;
        let tile_size = width * height * 3;
        let mut tile: Vec<int> = Vec::with_capacity(tile_size as uint);

        for y in range(from_y, to_y) {
            let inv_y = to_y - y;
            for x in range(from_x, to_x) {
                let ray = camera.get_ray(x, inv_y);
                let color = Renderer::trace(&scene, &ray, shadows, reflect_depth, refract_depth, false);
                tile.push((color.x.max(0.0).min(1.0) * 255.0) as int);
                tile.push((color.y.max(0.0).min(1.0) * 255.0) as int);
                tile.push((color.z.max(0.0).min(1.0) * 255.0) as int);
            }
        }

        tile
    }

    fn trace(scene: &Scene,
             ray: &Ray,
             shadows: bool,
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
                    let mut shadow = Vec3::one();
                    let l = (light.position() - nearest_hit.position).unit();

                    if (shadows) {
                        // L has to be unit vector for t_max 1:1 correspondence to
                        // distance to light to work. Shadow feelers only search up
                        // until light source
                        let shadow_ray = Ray {origin: nearest_hit.position, direction: l};
                        let distance_to_light = (light.position() - nearest_hit.position).len();

                        // Check against candidate primitives in scene for occlusion
                        // and multiply shadow color by occluders' shadow colors
                        shadow = scene.prims.iter().fold(Vec3::one(), |shadow_acc, prim| {
                            let occlusion = prim.intersects(&shadow_ray, epsilon, distance_to_light);
                            match occlusion {
                                Some(occulusion) => {shadow_acc * occulusion.material.transmission()}
                                None => shadow_acc
                            }
                        });
                    }

                    color_acc + light.color() * nearest_hit.material.sample(n, i, l) * shadow
                });

                // Global reflection
                // Something wrong here
                if nearest_hit.material.is_reflective() {
                    let r = Vec3::reflect(&i, &n);
                    let reflect_ray = Ray {origin: nearest_hit.position, direction: r};
                    let reflection = Renderer::trace(scene, &reflect_ray, shadows, reflect_depth - 1, refract_depth, inside);

                    result = result + nearest_hit.material.global_specular(&reflection);
                }

                // Global refraction
                if nearest_hit.material.is_refractive() {
                    let t = Vec3::refract(&i, &n, nearest_hit.material.ior(), inside);
                    let refract_ray = Ray {origin: nearest_hit.position + t.scale(epsilon), direction: t};
                    let refraction = Renderer::trace(scene, &refract_ray, shadows, reflect_depth, refract_depth, !inside);

                    result = result + nearest_hit.material.global_transmissive(&refraction);
                }

                result
            }

            None => {scene.background}
        }
    }
}
