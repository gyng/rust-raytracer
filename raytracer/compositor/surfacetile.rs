use raytracer::compositor::ColorRGBA;


#[allow(dead_code)]
pub struct SurfaceTile {
    pub width: uint,
    pub height: uint,
    pub x_off: uint,
    pub y_off: uint,
    pub buffer: Vec<ColorRGBA>
}


#[allow(dead_code)]
impl SurfaceTile {
    pub fn new(width: uint, height: uint, x_off: uint, y_off: uint, background: ColorRGBA) -> SurfaceTile {
        SurfaceTile {
            width: width,
            height: height,
            x_off: x_off,
            y_off: y_off,
            buffer: Vec::from_elem(width * height, background)
        }
    }


    #[inline]
    pub fn pixel_count(&self) -> uint {
        self.buffer.len()
    }

    #[inline]
    fn get_idx(&self, x: uint, y: uint) -> uint {
        if self.width <= x {
            fail!("`x` out of bounds (0 <= {} < {}", x, self.width);
        }
        if self.height <= y {
            fail!("`y` out of bounds (0 <= {} < {}", y, self.height);
        }
        self.width * y + x
    }

    #[inline]
    pub fn get(&self, x: uint, y: uint) -> ColorRGBA {
        let idx = self.get_idx(x, y);
        *self.buffer.get(idx)
    }

    #[inline]
    pub fn get_mut<'a>(&'a mut self, x: uint, y: uint) -> &'a mut ColorRGBA {
        let idx = self.get_idx(x, y);
        self.buffer.get_mut(idx)
    }
}