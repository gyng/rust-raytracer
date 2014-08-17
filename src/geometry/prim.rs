use geometry::BBox;
use raytracer::{Ray, Intersection};
use mat4::Transform;

pub trait Prim {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>>;
    fn bounding(&self) -> Option<BBox>;
    // fn transform(&self, transform: &Transform) -> Box<Prim+Send+Sync>;
    fn mut_transform(&mut self, transform: &Transform);
}
