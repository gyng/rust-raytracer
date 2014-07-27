use raytracer::compositor::{ColorRGBA, Surface};


pub struct SurfaceFactory {
    pub width: uint,
    pub height: uint,
    pub x_off: uint,
    pub y_off: uint,
    pub background: ColorRGBA<u8>
}


impl SurfaceFactory {
    pub fn new(width: uint, height: uint, x_off: uint, y_off: uint,
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
