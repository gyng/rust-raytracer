use material::textures::ImageTexture;
use vec3::Vec3;

#[allow(dead_code)]
pub struct CubeMap {
    pub faces: Vec<ImageTexture>
}

impl CubeMap {
    #[allow(dead_code)]
    pub fn load(x: &str, x_neg: &str, y: &str, y_neg: &str, z: &str, z_neg: &str) -> CubeMap {
        CubeMap {
            faces: vec![
                ImageTexture::load(x),
                ImageTexture::load(x_neg),
                ImageTexture::load(y),
                ImageTexture::load(y_neg),
                ImageTexture::load(z),
                ImageTexture::load(z_neg)
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

        // [-1..1] -> [0..1]
        s = s * 0.5 + 0.5;
        t = t * 0.5 + 0.5;

        self.faces[face].sample(s, t)
    }
}
