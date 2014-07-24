use material::textures::ImageTexture;
use vec3::Vec3;

#[allow(dead_code)]
pub struct CubeMap {
    pub faces: Vec<ImageTexture>
}

impl CubeMap {
    #[allow(dead_code)]
    pub fn load(front: &str, back: &str, up: &str, down: &str, left: &str, right: &str) -> CubeMap {
        CubeMap {
            faces: vec![
                ImageTexture {image: ::util::import::from_ppm(front)}, // +x
                ImageTexture {image: ::util::import::from_ppm(back)},  // -x
                ImageTexture {image: ::util::import::from_ppm(up)},    // +y
                ImageTexture {image: ::util::import::from_ppm(down)},  // -y
                ImageTexture {image: ::util::import::from_ppm(left)},  // +z
                ImageTexture {image: ::util::import::from_ppm(right)}  // -z
            ]
        }
    }

    #[allow(dead_code)]
    pub fn color(&self, dir: Vec3) -> Vec3 {
        // Select cube face
        let x_mag = dir.x.abs();
        let y_mag = dir.y.abs();
        let z_mag = dir.z.abs();

        let mut face = -1;
        let mut s = 0.0;
        let mut t = 0.0;

        if x_mag >= y_mag && x_mag > z_mag {
            // +x -x direction
            face = if dir.x < 0.0 { 0 } else { 1 };
            s = dir.y / dir.x;
            t = dir.z / dir.x;
        } else if y_mag >= x_mag && y_mag > z_mag {
            // +y -y direction
            face = if dir.y < 0.0 { 2 } else { 3 };
            s = dir.x / dir.y;
            t = dir.z / dir.y;
        } else if z_mag >= y_mag && z_mag > x_mag {
            // +z -z direction
            face = if dir.z < 0.0 { 4 } else { 5 };
            s = dir.x / dir.z;
            t = dir.y / dir.z;
        }

        // println!("skybox coords {} {}", s, t);

        // [-1..1] -> [0..1]
        s = s * 0.5 + 0.5;
        t = t * 0.5 + 0.5;

        self.faces[face].sample(s, t)
    }
}
