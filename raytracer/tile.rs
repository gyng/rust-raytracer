use vec3::Vec3;

pub struct Tile {
    pub from_x: int,
    pub from_y: int,
    pub to_x: int,
    pub to_y: int,
    pub data: Vec<Vec3>
}
