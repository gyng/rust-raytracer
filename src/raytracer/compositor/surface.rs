use std::cmp::min;
use std::iter::repeat;
use std::ops::{Index, IndexMut};

use raytracer::compositor::{ColorRGBA, SurfaceFactory};

#[derive(Clone)]
pub struct Surface {
    pub width: uint,
    pub height: uint,
    pub x_off: uint,
    pub y_off: uint,
    pub background: ColorRGBA<u8>,
    pub buffer: Vec<ColorRGBA<u8>>,
}


#[allow(dead_code)]
impl Surface {
    pub fn new(width: uint, height: uint, background: ColorRGBA<u8>) -> Surface {
        Surface {
            width: width,
            height: height,
            x_off: 0,
            y_off: 0,
            background: background,
            buffer: repeat(background).take(width * height).collect()
        }
    }

    pub fn with_offset(width: uint, height: uint, x_off: uint, y_off: uint,
                       background: ColorRGBA<u8>) -> Surface {
        Surface {
            width: width,
            height: height,
            x_off: x_off,
            y_off: y_off,
            background: background,
            buffer: repeat(background).take(width * height).collect()
        }
    }

    pub fn divide(&self, tile_width: uint, tile_height: uint) -> SubsurfaceIterator {
        SubsurfaceIterator {
            parent_width: self.width,
            parent_height: self.height,
            background: self.background,
            x_delta: tile_width,
            y_delta: tile_height,
            x_off: 0,
            y_off: 0,
        }
    }

    pub fn overrender_size(&self, tile_width: uint, tile_height: uint) -> (uint, uint) {
        let mut width = self.width;
        let width_partial_tile = width % tile_width;
        if width_partial_tile > 0 {
            width -= width_partial_tile;
            width += tile_width;
        }

        let mut height = self.height;
        let height_partial_tile = height % tile_height;
        if height_partial_tile > 0 {
            height -= height_partial_tile;
            height += tile_height;
        }

        (width, height)
    }

    pub fn merge(&mut self, tile: Box<Surface>) {
        let x_len: uint = min(tile.width, self.width - tile.x_off);
        let y_len: uint = min(tile.height, self.height - tile.y_off);

        for src_y in range(0, y_len) {
            let dst_y = tile.y_off + src_y;
            for src_x in range(0, x_len) {
                let dst_x = tile.x_off + src_x;
                self[(dst_x, dst_y)] = (&*tile)[(src_x, src_y)]
            }
        }
    }

    #[inline]
    pub fn pixel_count(&self) -> uint {
        self.buffer.len()
    }

    #[inline]
    fn get_idx(&self, x: uint, y: uint) -> uint {
        if self.width <= x {
            panic!("`x` out of bounds (0 <= {} < {}", x, self.width);
        }
        if self.height <= y {
            panic!("`y` out of bounds (0 <= {} < {}", y, self.height);
        }
        self.width * y + x
    }
}

impl Index<(uint, uint), ColorRGBA<u8>> for Surface {
    fn index<'a>(&'a self, index: &(uint, uint)) -> &'a ColorRGBA<u8> {
        let (x, y) = *index;
        let idx = self.get_idx(x, y);
        &self.buffer[idx]
    }
}

impl IndexMut<(uint, uint), ColorRGBA<u8>> for Surface {
    fn index_mut<'a>(&'a mut self, index: &(uint, uint)) -> &'a mut ColorRGBA<u8> {
        let (x, y) = *index;
        let idx = self.get_idx(x, y);
        &mut self.buffer[idx]
    }
}

struct SubsurfaceIterator {
    x_delta: uint,
    x_off: uint,
    y_delta: uint,
    y_off: uint,
    parent_width: uint,
    parent_height: uint,
    background: ColorRGBA<u8>,
}


impl SubsurfaceIterator {
    fn incr_tile(&mut self) {
        if self.x_off + self.x_delta < self.parent_width {
            self.x_off += self.x_delta;
        } else {
            self.x_off = 0;
            self.y_off += self.y_delta;
        }
    }

    fn current_tile(&self) -> Option<SurfaceFactory> {
        if self.x_off < self.parent_width && self.y_off < self.parent_height {
            Some(SurfaceFactory::new(
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

impl Iterator<SurfaceFactory> for SubsurfaceIterator {
    fn next(&mut self) -> Option<SurfaceFactory> {
        let tile = self.current_tile();
        self.incr_tile();
        tile
    }
}


#[test]
fn test_measurement() {
    let width = 800;
    let height = 600;
    let width_tile = 128;
    let height_tile = 8;

    let background: ColorRGBA<u8> = ColorRGBA::new_rgb(0, 0, 0);
    let surf: Surface = Surface::new(width, height, background);

    let mut total_pixels = 0;

    for tile_factory in surf.divide(width_tile, height_tile) {
        total_pixels += tile_factory.create().pixel_count();
    }

    let (or_width, or_height) = surf.overrender_size(width_tile, height_tile);

    assert_eq!(or_width * or_height, total_pixels);
}

#[test]
fn test_paint_it_red() {
    let width = 800;
    let height = 600;
    let width_tile = 128;
    let height_tile = 8;

    let background: ColorRGBA<u8> = ColorRGBA::new_rgb(0, 0, 0);
    let mut surf: Surface = Surface::new(width, height, background);

    for tile_factory in surf.divide(width_tile, height_tile) {
        let mut tile = tile_factory.create();
        for y in range(0, tile.height) {
            for x in range(0, tile.width) {
                tile[(x, y)] = ColorRGBA::new_rgb(255, 0, 0);
            }
        }
        for y in range(0, tile.height) {
            for x in range(0, tile.width) {
                assert_eq!(tile[(x, y)].r, 255);
                assert_eq!(tile[(x, y)].g, 0);
                assert_eq!(tile[(x, y)].b, 0);
            }
        }
        surf.merge(box tile);
    }

    for y in range(0, surf.height) {
        for x in range(0, surf.width) {
            let color = surf[(x, y)];
            if color.r != 255 {
                panic!("wrong pixel at {}x{}", x, y);
            }
            if color.g != 0 {
                panic!("wrong pixel at {}x{}", x, y);
            }
            if color.b != 0 {
                panic!("wrong pixel at {}x{}", x, y);
            }
        }
    }

    // Check the iterator too
    for color in surf.buffer.iter() {
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }
}
