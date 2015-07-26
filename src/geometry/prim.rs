use geometry::{BBox, PartialBoundingBox};
use raytracer::{Ray, Intersection};
use mat4::Transform;

pub trait Prim: PartialBoundingBox {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>>;
    // fn transform(&self, transform: &Transform) -> Box<Prim+Send+Sync>;
    fn mut_transform(&mut self, transform: &Transform);
}

impl<'a> PartialBoundingBox for Box<Prim+Send+Sync> {
	fn partial_bounding_box(&self) -> Option<BBox> {
		(**self).partial_bounding_box()
	}
}

impl<'a> PartialBoundingBox for &'a Box<Prim+Send+Sync> {
	fn partial_bounding_box(&self) -> Option<BBox> {
		(***self).partial_bounding_box()
	}
}