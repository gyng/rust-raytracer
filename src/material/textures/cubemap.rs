use material::textures::ImageTexture;
use std::sync::mpsc::channel;
use std::thread;
use vec3::Vec3;

#[allow(dead_code)]
pub struct CubeMap {
    pub faces: Vec<ImageTexture>
}

impl CubeMap {
    /// For y-axis as up, load: left, right, down, up, front, back
    #[allow(dead_code)]
    pub fn load(x: &str, x_neg: &str, y: &str, y_neg: &str, z: &str, z_neg: &str) -> CubeMap {
        let filenames = vec![
            x, x_neg,
            y, y_neg,
            z, z_neg,
        ];

        let mut faces: Vec<ImageTexture> = Vec::with_capacity(6);
        unsafe { faces.set_len(6); }

        let (tx, rx) = channel();

        for (i, filename) in filenames.iter().take(6).enumerate() {
            let task_tx = tx.clone();
            let filename = filename.to_string();

            thread::spawn(move || {
                task_tx.send((i, ImageTexture::load(&filename))).unwrap();
            });
        }
        drop(tx);

        for (i, tex) in rx {
            let p = faces.as_mut_ptr();
            unsafe { ::std::ptr::write::<ImageTexture>(p.offset(i as isize), tex); }
        }

        CubeMap { faces: faces }
    }

    #[allow(dead_code)]
    pub fn color(&self, dir: Vec3) -> Vec3 {
        let x_mag = dir.x.abs();
        let y_mag = dir.y.abs();
        let z_mag = dir.z.abs();

        let mut face = !0;
        let mut s = 0.0;
        let mut t = 0.0;

        if x_mag >= y_mag && x_mag >= z_mag {
            // +x -x direction
            face = if dir.x <= 0.0 { 0 } else { 1 };
            let scale = if dir.x < 0.0 { 1.0 } else { -1.0 };
            s = scale * dir.z / dir.x.abs();
            t = dir.y / dir.x.abs();
        } else if y_mag >= x_mag && y_mag >= z_mag {
            // +y -y direction
            face = if dir.y <= 0.0 { 2 } else { 3 };
            let scale = if dir.y < 0.0 { 1.0 } else { -1.0 };
            s = scale * dir.x / dir.y.abs();
            t = dir.z / dir.y.abs();
        } else if z_mag >= y_mag && z_mag >= x_mag {
            // +z -z direction
            face = if dir.z <= 0.0 { 4 } else { 5 };
            let scale = if dir.z < 0.0 { -1.0 } else { 1.0 };
            s = scale * dir.x / dir.z.abs();
            t = dir.y / dir.z.abs();
        }

        // [-1..1] -> [0..1]
        let seam_delta = 0.0001;
        s = (1.0 - (s * 0.5 + 0.5)).max(seam_delta).min(1.0 - seam_delta);
        t = (1.0 - (t * 0.5 + 0.5)).max(seam_delta).min(1.0 - seam_delta);

        if face == !0 {
            panic!("CubeMap could not get a cube face for direction {} {} {}", dir.x, dir.y, dir.z);
        }

        self.faces[face].sample(s, t)
    }
}
