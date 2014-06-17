use raytracer::Tile;

pub use self::colorrgba::ColorRGBA;
pub use self::surface::Surface;
pub use self::surfacetile::SurfaceTile;
pub use self::surfacetilefactory::SurfaceTileFactory;
pub use self::surfacetileiterator::SurfaceTileIterator;

pub mod colorrgba;
pub mod surface;
pub mod surfacetile;
pub mod surfacetilefactory;
pub mod surfacetileiterator;

/// Takes in Tiles and returns a combined Vec<Vec3>.
pub fn composite(tiles: Vec<Tile>, image_width: int, image_height: int) -> Surface {
    let mut result: Surface = Surface::new(
        image_width as uint,
        image_height as uint,
        ColorRGBA::black());

    for tile in tiles.iter() {
        let tile_width = tile.to_x - tile.from_x;

        for pixel_y in range(tile.from_y, tile.to_y) {
            let inv_pixel_y = image_height - pixel_y - 1;

            for pixel_x in range(tile.from_x, tile.to_x) {
                let tile_pixel_x = pixel_x - tile.from_x;
                let tile_pixel_y = pixel_y - tile.from_y;
                let tile_pixel_index = tile_pixel_x + tile_pixel_y * tile_width;

                *result.get_mut(pixel_x as uint, inv_pixel_y as uint) =
                    *tile.data.get(tile_pixel_index as uint);
            }
        }
    }
    result
}
