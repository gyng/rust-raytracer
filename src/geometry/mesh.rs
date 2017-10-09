use geometry::Prim;
use mat4::Transform;

#[allow(dead_code)]
pub struct Mesh {
    pub triangles: Vec<Box<Prim+Send+Sync>>
}

impl Mesh {
    pub fn mut_transform(&mut self, transform: &Transform) {
        for triangle in &mut self.triangles {
            triangle.mut_transform(transform);
        }
    }
}
