use raytracer::compositor::{ColorRGBA, SurfaceTileFactory};


pub struct SurfaceTileIterator {
    x_delta: uint,
    x_off: uint,
    y_delta: uint,
    y_off: uint,
    parent_width: uint,
    parent_height: uint,
    background: ColorRGBA,
}


impl SurfaceTileIterator {
    fn incr_tile(&mut self) {
        if self.x_off + self.x_delta < self.parent_width {
            self.x_off += self.x_delta;
        } else {
            self.x_off = 0;
            self.y_off += self.y_delta;
        }
    }

    fn current_tile(&self) -> Option<SurfaceTileFactory> {
        if self.x_off < self.parent_width && self.y_off < self.parent_height {
            Some(SurfaceTileFactory::new(
                self.x_delta,
                self.y_delta,
                self.x_off,
                self.y_off,
                self.background
            ))
        } else {
            None
        }
    }
}

impl Iterator<SurfaceTileFactory> for SurfaceTileIterator {
    fn next(&mut self) -> Option<SurfaceTileFactory> {
        let tile = self.current_tile();
        self.incr_tile();
        tile
    }
}