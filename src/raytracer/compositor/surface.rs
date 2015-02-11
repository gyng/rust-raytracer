use std::cmp::min;
use std::iter::repeat;
use std::ops::{Index, IndexMut};

use raytracer::compositor::{ColorRGBA, SurfaceFactory};

#[derive(Clone)]
pub struct Surface {
    pub width: usize,
    pub height: usize,
    pub x_off: usize,
    pub y_off: usize,
    pub background: ColorRGBA<u8>,
    pub buffer: Vec<ColorRGBA<u8>>,
}


#[allow(dead_code)]
impl Surface {
    pub fn new(width: usize, height: usize, background: ColorRGBA<u8>) -> Surface {
        Surface {
            width: width,
            height: height,
            x_off: 0,
            y_off: 0,
            background: background,
            buffer: repeat(background).take(width * height).collect()
        }
    }

    pub fn with_offset(width: usize, height: usize, x_off: usize, y_off: usize,
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

    pub fn divide(&self, tile_width: usize, tile_height: usize) -> SubsurfaceIterator {
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

    pub fn overrender_size(&self, tile_width: usize, tile_height: usize) -> (usize, usize) {
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

    pub fn merge(&mut self, tile: &Surface) {
        let x_len: usize = min(tile.width, self.width - tile.x_off);
        let y_len: usize = min(tile.height, self.height - tile.y_off);

        for src_y in range(0, y_len) {
            let dst_y = tile.y_off + src_y;
            for src_x in range(0, x_len) {
                let dst_x = tile.x_off + src_x;
                self[(dst_x, dst_y)] = tile[(src_x, src_y)]
            }
        }
    }

    #[inline]
    pub fn pixel_count(&self) -> usize {
        self.buffer.len()
    }

    #[inline]
    fn get_idx(&self, x: usize, y: usize) -> usize {
        if self.width <= x {
            panic!("`x` out of bounds (0 <= {} < {}", x, self.width);
        }
        if self.height <= y {
            panic!("`y` out of bounds (0 <= {} < {}", y, self.height);
        }
        self.width * y + x
    }
}

impl Index<(usize, usize)> for Surface {
    type Output = ColorRGBA<u8>;

    fn index<'a>(&'a self, index: &(usize, usize)) -> &'a ColorRGBA<u8> {
        let (x, y) = *index;
        let idx = self.get_idx(x, y);
        &self.buffer[idx]
    }
}

impl IndexMut<(usize, usize)> for Surface {
    fn index_mut<'a>(&'a mut self, index: &(usize, usize)) -> &'a mut ColorRGBA<u8> {
        let (x, y) = *index;
        let idx = self.get_idx(x, y);
        &mut self.buffer[idx]
    }
}

struct SubsurfaceIterator {
    x_delta: usize,
    x_off: usize,
    y_delta: usize,
    y_off: usize,
    parent_width: usize,
    parent_height: usize,
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

impl Iterator for SubsurfaceIterator {
    type Item = SurfaceFactory;

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
        surf.merge(&tile);
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
