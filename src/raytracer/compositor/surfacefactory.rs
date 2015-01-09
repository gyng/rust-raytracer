use raytracer::compositor::{ColorRGBA, Surface};


pub struct SurfaceFactory {
    pub width: usize,
    pub height: usize,
    pub x_off: usize,
    pub y_off: usize,
    pub background: ColorRGBA<u8>
}


impl SurfaceFactory {
    pub fn new(width: usize, height: usize, x_off: usize, y_off: usize,
               background: ColorRGBA<u8>) -> SurfaceFactory {
        SurfaceFactory {
            width: width,
            height: height,
            x_off: x_off,
            y_off: y_off,
            background: background
        }
    }

    #[allow(dead_code)]
    pub fn create(&self) -> Surface {
        Surface::with_offset(self.width, self.height, self.x_off, self.y_off, self.background)
    }
}
