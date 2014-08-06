#![allow(dead_code)]

use geometry::bbox::{union_point, union_points, BBox};
use geometry::prim::Prim;
use material::Material;
use raytracer::{Ray, Intersection};
use vec3::Vec3;

#[cfg(test)]
use material::materials::FlatMaterial;


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
    /// http://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    /// Barycentric coordinates.
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>> {
        let e1 = self.v1.pos - self.v0.pos;
        let e2 = self.v2.pos - self.v0.pos;
        let p = ray.direction.cross(&e2);
        let det = e1.dot(&p);

        // if determinant is near zero, ray lies in plane of triangle
        if det > -::std::f64::EPSILON && det < ::std::f64::EPSILON {
            return None
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - self.v0.pos;
        let beta = inv_det * s.dot(&p);
        if beta < 0.0 || beta > 1.0 { return None }

        let q = s.cross(&e1);
        let gamma = inv_det * ray.direction.dot(&q);
        if gamma < 0.0 || beta + gamma > 1.0 { return None }

        let t = inv_det * e2.dot(&q);

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

#[test]
fn it_intersects_and_interpolates() {
    let triangle = Triangle {
        v0: TriangleVertex {pos: Vec3 {x: -1.0, y: 0.0, z: 0.0}, n: Vec3 {x: -1.0, y: 0.0, z: 0.0}, u: 0.0, v: 0.0},
        v1: TriangleVertex {pos: Vec3 {x:  1.0, y: 0.0, z: 0.0}, n: Vec3 {x:  1.0, y: 0.0, z: 0.0}, u: 1.0, v: 0.0},
        v2: TriangleVertex {pos: Vec3 {x:  0.0, y: 1.0, z: 0.0}, n: Vec3 {x:  0.0, y: 1.0, z: 0.0}, u: 0.0, v: 1.0},
        material: box FlatMaterial {color: Vec3::one()}
    };

    // Tests actual intersection
    let intersecting_ray = Ray::new(Vec3 {x: 0.0, y: 0.5, z: -1.0}, Vec3 {x: 0.0, y: 0.0, z: 1.0});
    let intersection = triangle.intersects(&intersecting_ray, 0.0, 10.0).unwrap();
    assert_eq!(intersection.position.x, 0.0);
    assert_eq!(intersection.position.y, 0.5);
    assert_eq!(intersection.position.z, 0.0);
    assert_eq!(intersection.u, 0.25);
    assert_eq!(intersection.v, 0.5);
    assert_eq!(intersection.n.x, 0.0);
    assert_eq!(intersection.n.y, 0.5);
    assert_eq!(intersection.n.z, 0.0);

    // Ray off to the sides
    let mut non_intersecting_ray = Ray::new(Vec3 {x: 0.0, y: 0.5, z: -1.0}, Vec3 {x: 100.0, y: 100.0, z: 1.0});
    let mut non_intersection = triangle.intersects(&non_intersecting_ray, 0.0, 10.0);
    assert!(non_intersection.is_none());

    non_intersecting_ray = Ray::new(Vec3 {x: 0.0, y: 0.5, z: -1.0}, Vec3 {x: -100.0, y: -100.0, z: 1.0});
    non_intersection = triangle.intersects(&non_intersecting_ray, 0.0, 10.0);
    assert!(non_intersection.is_none());

    // Ray in opposite direction
    non_intersecting_ray = Ray::new(Vec3 {x: 0.0, y: 0.5, z: -1.0}, Vec3 {x: 0.0, y: 0.0, z: -1.0});
    non_intersection = triangle.intersects(&non_intersecting_ray, 0.0, 10.0);
    assert!(non_intersection.is_none());
}

#[test]
fn it_intersects_only_in_tmin_tmax() {
    let triangle = Triangle {
        v0: TriangleVertex {pos: Vec3 {x: -1.0, y: 0.0, z: 0.0}, n: Vec3::zero(), u: 0.0, v: 0.0},
        v1: TriangleVertex {pos: Vec3 {x: 1.0, y: 0.0, z: 0.0},  n: Vec3::zero(), u: 1.0, v: 0.0},
        v2: TriangleVertex {pos: Vec3 {x: 0.0, y: 1.0, z: 0.0},  n: Vec3::one(),  u: 0.0, v: 1.0},
        material: box FlatMaterial {color: Vec3::one()}
    };

    // Tests tmin
    let mut intersecting_ray = Ray::new(Vec3 {x: 0.0, y: 0.5, z: -1.0}, Vec3 {x: 0.0, y: 0.0, z: 1.0});
    let mut non_intersection = triangle.intersects(&intersecting_ray, 1000.0, 10000.0);
    assert!(non_intersection.is_none());

    // Tests tmax
    non_intersection = triangle.intersects(&intersecting_ray, 0.0, 0.0001);
    assert!(non_intersection.is_none());
}
