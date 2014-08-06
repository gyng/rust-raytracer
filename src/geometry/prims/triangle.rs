#![allow(dead_code)]

use geometry::bbox::{union_point, union_points, BBox};
use geometry::prim::Prim;
use material::Material;
use raytracer::{Ray, Intersection};
use vec3::Vec3;


pub struct Triangle {
    pub v0: TriangleVertex,
    pub v1: TriangleVertex,
    pub v2: TriangleVertex,
    pub material: Box<Material+Send+Share>
}


pub struct TriangleVertex {
    pub pos: Vec3,
    pub n: Vec3,
    pub u: f64,
    pub v: f64
}


impl Triangle {
    /// All three normals at vertices are perpendicular to the triangle plane
    pub fn auto_normal(v0: Vec3, v1: Vec3, v2: Vec3, uv0: (f64, f64), uv1: (f64, f64), uv2: (f64, f64), material: Box<Material+Send+Share>) -> Triangle {
        let n = (v1 - v0).cross(&(v2 - v0));
        let (ut0, vt0) = uv0;
        let (ut1, vt1) = uv1;
        let (ut2, vt2) = uv2;

        Triangle {
            v0: TriangleVertex{ pos: v0, n: n, u: ut0, v: vt0 },
            v1: TriangleVertex{ pos: v1, n: n, u: ut1, v: vt1 },
            v2: TriangleVertex{ pos: v2, n: n, u: ut2, v: vt2 },
            material: material
        }
    }
}


impl Prim for Triangle {
    /// Barycentric coordinates.
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>> {
        let e1 = self.v1.pos - self.v0.pos;
        let e2 = self.v2.pos - self.v0.pos;
        let p = ray.direction.cross(&e2);
        let a = e1.dot(&p);

        let f = 1.0 / a;
        let s = ray.origin - self.v0.pos;
        let beta = f * s.dot(&p);
        if beta < 0.0 || beta > 1.0 { return None }

        let q = s.cross(&e1);
        let gamma = f * ray.direction.dot(&q);
        if gamma < 0.0 || beta + gamma > 1.0 { return None }

        let t = f * e2.dot(&q);

        if t < t_min || t > t_max {
            None
        } else {
            let intersection_point = ray.origin + ray.direction.scale(t);

            let alpha = 1.0 - beta - gamma;

            // Interpolate normals at vertices to get normal
            let n = self.v0.n.scale(alpha) + self.v1.n.scale(beta) + self.v2.n.scale(gamma);

            // Interpolate UVs at vertices to get UV
            let u = self.v0.u * alpha + self.v1.u * beta + self.v2.u * gamma;
            let v = self.v0.v * alpha + self.v1.v * beta + self.v2.v * gamma;

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
        return Some(union_point(&union_points(&self.v0.pos, &self.v1.pos), &self.v2.pos));
    }
}
