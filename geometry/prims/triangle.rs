use geometry::bbox::{union_point, union_points, BBox};
use geometry::prim::Prim;
use material::Material;
use raytracer::{Ray, Intersection};
use vec3::Vec3;

#[allow(dead_code)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub n0: Vec3,
    pub n1: Vec3,
    pub n2: Vec3,
    // Texture (u, v) coordinates
    pub u: Vec3, // Vec3 {x: u0, y: u1, z: u2}
    pub v: Vec3, // Vec3 {x: v0, y: v1, z: v2}
    pub material: Box<Material+Send+Share>
}

impl Triangle {
    /// All three normals at vertices are perpendicular to the triangle plane
    #[allow(dead_code)]
    pub fn auto_normal(v0: Vec3, v1: Vec3, v2: Vec3, u: Vec3, v: Vec3, material: Box<Material+Send+Share>) -> Triangle {
        // let n = (v1 - v0).cross(&(v2 - v0));
        let n = (v1 - v0).cross(&(v2 - v0));
        Triangle {
            v0: v0,
            v1: v1,
            v2: v2,
            n0: n,
            n1: n,
            n2: n,
            u: u,
            v: v,
            material: material
        }
    }
}

impl Prim for Triangle {
    /// Barycentric coordinates.
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>> {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        let p = ray.direction.cross(&e2);
        let a = e1.dot(&p);

        let f = 1.0 / a;
        let s = ray.origin - self.v0;
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
            let n = self.n0.scale(alpha) + self.n1.scale(beta) + self.n2.scale(gamma);

            // Interpolate UVs at vertices to get UV
            let u = self.u.x * alpha + self.u.y * beta + self.u.z * gamma;
            let v = self.v.x * alpha + self.v.y * beta + self.v.z * gamma;

            Some(Intersection {
                n: n,
                t: t,
                u: u,
                v: v,
                position: intersection_point,
                material: &'a self.material
            })
        }
    }

    fn bounding(&self) -> Option<BBox> {
        return Some(union_point(&union_points(&self.v0, &self.v1), &self.v2));
    }
}
