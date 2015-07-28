#![allow(dead_code)]

use geometry::bbox::{union_point, union_points, BBox, PartialBoundingBox};
use geometry::prim::Prim;
use material::Material;
use mat4::{Mat4, Transform};
use raytracer::{Ray, Intersection};
use vec3::Vec3;

use material::materials::FlatMaterial;


struct UvValue {
    u: f64,
    v: f64
}

impl UvValue {
    pub fn from_tuple(uv: (f64, f64)) -> UvValue {
        UvValue { u: uv.0, v: uv.1 }
    }

    fn default3() -> [UvValue; 3] {
        [
            UvValue { u: 0.5, v: 1.0 },
            UvValue { u: 0.0, v: 0.0 },
            UvValue { u: 1.0, v: 0.0 },
        ]
    }
}

pub struct TriangleOptions {
    vertices: [Vec3; 3],
    normals: Option<[Vec3; 3]>,
    texinfo: Option<[UvValue; 3]>,
    material: Option<Box<Material+Send+Sync>>,
}

fn get_auto_normals(v: [Vec3; 3]) -> [Vec3; 3] {
    let n = (v[1] - v[0]).cross(&(v[2] - v[0]));
    [n, n, n]
}

impl TriangleOptions {   
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> TriangleOptions {
        TriangleOptions {
            vertices: [v0, v1, v2],
            normals: None,
            texinfo: None,
            material: None,
        }
    }

    /// In the default case, all three normals at vertices are perpendicular
    /// to the triangle plane.
    pub fn normals(&mut self, normals: [Vec3; 3]) -> &mut Self {
        self.normals = Some(normals);
        self
    }

    pub fn texinfo(&mut self, texinfo: [(f64, f64); 3]) -> &mut Self {
        self.texinfo = Some([
            UvValue::from_tuple(texinfo[0]),
            UvValue::from_tuple(texinfo[1]),
            UvValue::from_tuple(texinfo[2]),
        ]);
        self
    }

    pub fn material(&mut self, material: Box<Material+Send+Sync>) -> &mut Self {
        self.material = Some(material);
        self
    }

    pub fn build(self) -> Triangle {
        let normals = self.normals.unwrap_or_else(|| get_auto_normals(self.vertices));
        let texinfo = self.texinfo.unwrap_or_else(UvValue::default3);
        let material = self.material.unwrap_or_else(|| Box::new(FlatMaterial { color: Vec3::one() }));

        Triangle {
            vertices: self.vertices,
            normals: normals,
            texinfo: texinfo,
            material: material,
        }
    }
}

pub struct Triangle {
    vertices: [Vec3; 3],

    // All the same if our triangle is ``flat''.
    // Values differ when we want interpolation. e.g. round things like teapot.
    normals: [Vec3; 3],

    // Used in textured triangles, can be [UvValue; 3]::default() otherwise.
    texinfo: [UvValue; 3],

    material: Box<Material+Send+Sync>
}

impl PartialBoundingBox for Triangle {
    fn partial_bounding_box(&self) -> Option<BBox> {
        Some(union_point(&union_points(&self.vertices[0], &self.vertices[1]), &self.vertices[2]))
    }
}

impl Prim for Triangle {
    /// http://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    /// Barycentric coordinates.
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>> {
        let e1 = self.vertices[1] - self.vertices[0];
        let e2 = self.vertices[2] - self.vertices[0];
        let p = ray.direction.cross(&e2);
        let det = e1.dot(&p);

        // if determinant is near zero, ray lies in plane of triangle
        if det > -::std::f64::EPSILON && det < ::std::f64::EPSILON {
            return None
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - self.vertices[0];
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
            let n = self.normals[0].scale(alpha) + self.normals[1].scale(beta) + self.normals[2].scale(gamma);

            // Interpolate UVs at vertices to get UV
            let u = self.texinfo[0].u * alpha + self.texinfo[1].u * beta + self.texinfo[2].u * gamma;
            let v = self.texinfo[0].v * alpha + self.texinfo[1].v * beta + self.texinfo[2].v * gamma;

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

    fn mut_transform(&mut self, transform: &Transform) {
        let v0_t = Mat4::mult_p(&transform.m, &self.vertices[0]);
        let v1_t = Mat4::mult_p(&transform.m, &self.vertices[1]);
        let v2_t = Mat4::mult_p(&transform.m, &self.vertices[2]);

        let n0_t = Mat4::transform_normal(&self.normals[0], &transform.m);
        let n1_t = Mat4::transform_normal(&self.normals[1], &transform.m);
        let n2_t = Mat4::transform_normal(&self.normals[2], &transform.m);

        self.vertices[0] = v0_t;
        self.vertices[1] = v1_t;
        self.vertices[2] = v2_t;

        self.normals[0] = n0_t;
        self.normals[1] = n1_t;
        self.normals[2] = n2_t;
    }
}

#[test]
fn it_intersects_and_interpolates() {
    let mut triopts = TriangleOptions::new(
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Vec3 { x:  1.0, y: 0.0, z: 0.0 },
        Vec3 { x:  0.0, y: 1.0, z: 0.0 });
    triopts.normals([
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Vec3 { x:  1.0, y: 0.0, z: 0.0 },
        Vec3 { x:  0.0, y: 1.0, z: 0.0 }]);
    triopts.texinfo([(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);

    let triangle = triopts.build();

    // Tests actual intersection
    let intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 0.5, z: -1.0 }, Vec3 { x: 0.0, y: 0.0, z: 1.0 });
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
    let mut non_intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 0.5, z: -1.0 }, Vec3 { x: 100.0, y: 100.0, z: 1.0 });
    let mut non_intersection = triangle.intersects(&non_intersecting_ray, 0.0, 10.0);
    assert!(non_intersection.is_none());

    non_intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 0.5, z: -1.0 }, Vec3 { x: -100.0, y: -100.0, z: 1.0 });
    non_intersection = triangle.intersects(&non_intersecting_ray, 0.0, 10.0);
    assert!(non_intersection.is_none());

    // Ray in opposite direction
    non_intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 0.5, z: -1.0 }, Vec3 { x: 0.0, y: 0.0, z: -1.0 });
    non_intersection = triangle.intersects(&non_intersecting_ray, 0.0, 10.0);
    assert!(non_intersection.is_none());
}

#[test]
fn it_intersects_only_in_tmin_tmax() {
    let mut triopts = TriangleOptions::new(
        Vec3 { x: -1.0, y: 0.0, z: 0.0 },
        Vec3 { x:  1.0, y: 0.0, z: 0.0 },
        Vec3 { x:  0.0, y: 1.0, z: 0.0 });
    triopts.normals([Vec3::zero(), Vec3::zero(), Vec3::one()]);
    triopts.texinfo([(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);

    let triangle = triopts.build();

    // Tests tmin
    let intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 0.5, z: -1.0 }, Vec3 { x: 0.0, y: 0.0, z: 1.0 });
    let mut non_intersection = triangle.intersects(&intersecting_ray, 1000.0, 10000.0);
    assert!(non_intersection.is_none());

    // Tests tmax
    non_intersection = triangle.intersects(&intersecting_ray, 0.0, 0.0001);
    assert!(non_intersection.is_none());
}
