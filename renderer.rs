use camera::Camera;
use scene::Scene;
use ray::Ray;
use vec3::Vec3;

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

        // let (tx, rx): (Sender<Vec<int>>, Receiver<Vec<int>>) = channel();

        // // Copy camera and scene for now for each thread
        // // Use a per-tile task -- don't see utility in a per-trace/per-pixel task
        // // due to low processor count
        // for thread_no in range(0, 1) {
        //     let child_tx = tx.clone();

        //     spawn(proc() {
        //         let result = Renderer::render_tile(camera, scene, 0, 0, camera.image_width, camera.image_height);
        //         child_tx.send(result);
        //     });
        // }

        // // TODO: Composite tiles
        // // for range(0, 1) {
        //     let mut composite = rx.recv();
        // // }

        // composite

        Renderer::render_tile(camera, scene, true, 0, 0, camera.image_width, camera.image_height)
    }

    fn render_tile(camera: Camera,
                   scene: Scene,
                   shadows: bool,
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
                // Hardcoded reflect/refract depth, octree to come
                let color = Renderer::trace(&scene, &ray, shadows, 2, 4);

                // TODO: factor out floor to avoid premature precision loss
                // let index = ((x - from_x) * 3) + ((y - from_y) * width * 3);
                tile.push((color.x * 255.0) as int);
                tile.push((color.y * 255.0) as int);
                tile.push((color.z * 255.0) as int);
            }
        }

        tile
    }

    fn trace(scene: &Scene,
             ray: &Ray,
             shadows: bool,
             reflect_depth: int,
             refract_depth: int)
             -> Vec3 {
        if reflect_depth <= 0 || refract_depth <= 0 {
            return Vec3 {x: 0.0, y: 0.0, z: 0.0}
        }

        let nearest_hit = ray.get_nearest_hit(scene);

        match nearest_hit {
            Some(nearest_hit) => {
                let mut result = Vec3 {x: 0.0, y: 0.0, z: 0.0};
                let mut shadow = Vec3 {x: 1.0, y: 1.0, z: 1.0};

                let n = nearest_hit.n.unit();
                let i = (ray.direction.scale(-1.0)).unit();

                // Local lighting computation: surface shading, shadows
                for light in scene.lights.iter() {
                    let l = (light.position() - nearest_hit.position).unit();

                    if (shadows) {
                        // L has to be unit vector for t_max 1:1 correspondence to
                        // distance to light to work. Shadow feelers only search up
                        // until light source
                        let shadow_ray = Ray {origin: nearest_hit.position, direction: l};
                        let distance_to_light = (light.position() - nearest_hit.position).len();

                        // Check against candidate primitives in scene for occlusion
                        // and multiply shadow color by occluders' shadow colors
                        for prim in scene.prims.iter() {
                            let occulusion = prim.intersects(&shadow_ray, 0.0001, distance_to_light);
                            shadow = match occulusion {
                                Some(occulusion) => {shadow * occulusion.material.transmission()}
                                None => {shadow}
                            }
                        }
                    }

                    result = result + light.color() * nearest_hit.material.sample(n, i, l) * shadow;
                }

                // Global reflection
                if nearest_hit.material.is_specular() {
                    let r = Vec3::reflect(&ray.direction.scale(-1.0), &n);
                    let reflect_ray = Ray{origin: nearest_hit.position, direction: r};
                    let reflection = Renderer::trace(scene, &reflect_ray, shadows, reflect_depth - 1, refract_depth);

                    result = result + nearest_hit.material.global_specular(&reflection);
                }

                result
            }

            None => {
                scene.background
            }
        }
    }
}
