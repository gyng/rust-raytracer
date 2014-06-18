use geometry::BBox;
use raytracer::{Ray, Intersection};

pub trait Prim {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>>;
    fn bounding(&self) -> BBox;
}
