use geometry::{BBox, Prim};
use material::Material;
use raytracer::{Ray, Intersection};
use vec3::Vec3;

#[allow(dead_code)]
// Plane is defined as N dot D
pub struct Plane {
    pub a: f64, // normal.x
    pub b: f64, // normal.y
    pub c: f64, // normal.z
    pub d: f64,
    pub material: Box<Material+Send+Share>
}

impl Prim for Plane {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>> {
        let n = Vec3 {x: self.a, y: self.b, z: self.c};
        let nrd = n.dot(&ray.direction);
        let nro = n.dot(&ray.origin);
        let t = (-self.d - nro) / nrd;

        if t < t_min || t > t_max {
            None
        } else {
            let intersection_point = ray.origin + ray.direction.scale(t);

            let u_axis = Vec3 {x: n.y, y: n.z, z: -n.x};
            let v_axis = u_axis.cross(&n);
            let u = intersection_point.dot(&u_axis);
            let v = intersection_point.dot(&v_axis);

            Some(Intersection {
                n: n,
                t: t,
                u: u,
                v: v,
                position: intersection_point,
                material: &self.material
            })
        }
    }

    fn bounding(&self) -> Option<BBox> {
        None // more infinite than infinityb
    }
}
