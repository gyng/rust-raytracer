use raytracer::compositor::{ColorRGBA, SurfaceTile};


pub struct SurfaceTileFactory {
    pub width: uint,
    pub height: uint,
    pub x_off: uint,
    pub y_off: uint,
    pub background: ColorRGBA
}


impl SurfaceTileFactory {
    pub fn new(width: uint, height: uint, x_off: uint, y_off: uint, background: ColorRGBA) -> SurfaceTileFactory {
        SurfaceTileFactory {
            width: width,
            height: height,
            x_off: x_off,
            y_off: y_off,
            background: background
        }
    }

    #[allow(dead_code)]
    pub fn create(&self) -> Box<SurfaceTile> {
        box SurfaceTile::new(self.width, self.height, self.x_off, self.y_off, self.background)
    }
}
