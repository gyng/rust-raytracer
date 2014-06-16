use raytracer::Tile;
use vec3::Vec3;

/// Takes in Tiles and returns a combined Vec<Vec3>.
pub fn composite(tiles: Vec<Tile>, image_width: int, image_height: int) -> Vec<Vec3> {
    let mut result: Vec<Vec3> = Vec::from_elem((image_width * image_height) as uint, Vec3::zero());

    for tile in tiles.iter() {
        let tile_width = tile.to_x - tile.from_x;

        for pixel_y in range(tile.from_y, tile.to_y) {
            let inv_pixel_y = image_height - pixel_y - 1;

            for pixel_x in range(tile.from_x, tile.to_x) {
                let index = pixel_x + inv_pixel_y * image_width;

                let tile_pixel_x = pixel_x - tile.from_x;
                let tile_pixel_y = pixel_y - tile.from_y;
                let tile_pixel_index = tile_pixel_x + tile_pixel_y * tile_width;

                *result.get_mut(index as uint) = *tile.data.get(tile_pixel_index as uint);
            }
        }
    }

    result
}
