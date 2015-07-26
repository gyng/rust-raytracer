use geometry::bbox::{BBox, PartialBoundingBox};
use geometry::prim::Prim;
use material::Material;
use mat4::{Mat4, Transform};
use raytracer::{Ray, Intersection};
use vec3::Vec3;

#[cfg(test)]
use material::materials::FlatMaterial;

#[allow(dead_code)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<Material+Send+Sync>
}

impl PartialBoundingBox for Sphere {
    fn partial_bounding_box(&self) -> Option<BBox> {
        Some(BBox {
            min: Vec3 {
                x: self.center.x - self.radius,
                y: self.center.y - self.radius,
                z: self.center.z - self.radius
            },
            max: Vec3 {
                x: self.center.x + self.radius,
                y: self.center.y + self.radius,
                z: self.center.z + self.radius
            }
        })
    }
}

impl Prim for Sphere {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>> {
        let i = ray.origin - self.center;
        let a = 1.0;
        let b = 2.0 * ray.direction.dot(&i);
        let c = i.dot(&i) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant <= 0.0 {
            None
        } else {
            // Up to two intersections
            let disc_sqrt = discriminant.sqrt();
            let t1 = (-b + disc_sqrt) / 2.0 * a;
            let t2 = (-b - disc_sqrt) / 2.0 * a;

            if t1 >= t_min && t1 <= t_max ||
               t2 >= t_min && t2 <= t_max {
                // Valid intersection(s): get nearer intersection
                let t = if t1.abs() < t2.abs() { t1 } else { t2 };
                let intersection_point = ray.origin + ray.direction.scale(t);
                let n = (intersection_point - self.center).unit();

                let u = 0.5 + n.z.atan2(n.x) / (::std::f64::consts::PI * 2.0);
                let v = 0.5 - n.y.asin() / ::std::f64::consts::PI;

                Some(Intersection {
                    n: n,
                    t: t,
                    u: u,
                    v: v,
                    position: intersection_point,
                    material: &self.material
                })
            } else {
                None
            }
        }
    }

    fn mut_transform(&mut self, transform: &Transform) {
        let new_center = Mat4::mult_p(&transform.m, &self.center);

        let new_radius = if transform.m.has_scale() {
            self.radius * transform.m.scale()
        } else {
            self.radius
        };

        self.center = new_center;
        self.radius = new_radius;
    }
}

#[test]
fn it_intersects() {
    let sphere = Sphere {
        center: Vec3::zero(),
        radius: 1.0,
        material: Box::new(FlatMaterial { color: Vec3::one() })
    };

    // Tests actual intersection
    let intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 0.0, z: -2.0 }, Vec3 { x: 0.0, y: 0.0, z: 1.0 });
    let intersection = sphere.intersects(&intersecting_ray, 0.0, 10.0).unwrap();
    assert_eq!(intersection.position.x, 0.0);
    assert_eq!(intersection.position.y, 0.0);
    assert_eq!(intersection.position.z, -1.0);
    assert_eq!(intersection.n.x, 0.0);
    assert_eq!(intersection.n.y, 0.0);
    assert_eq!(intersection.n.z, -1.0);

    // Ray off to the sides
    let mut non_intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 0.0, z: -2.0 }, Vec3 { x: 100.0, y: 100.0, z: 0.1 });
    let mut non_intersection = sphere.intersects(&non_intersecting_ray, 0.0, 10.0);
    assert!(non_intersection.is_none());

    non_intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 0.0, z: -2.0 }, Vec3 { x: -100.0, y: -100.0, z: 0.1 });
    non_intersection = sphere.intersects(&non_intersecting_ray, 0.0, 10.0);
    assert!(non_intersection.is_none());

    // Ray in opposite direction
    non_intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 0.0, z: -2.0 }, Vec3 {x: 0.0, y: 0.0, z: -1.0 });
    non_intersection = sphere.intersects(&non_intersecting_ray, 0.0, 10.0);
    assert!(non_intersection.is_none());
}

#[test]
fn it_intersects_only_in_tmin_tmax() {
    let sphere = Sphere {
        center: Vec3::zero(),
        radius: 1.0,
        material: Box::new(FlatMaterial { color: Vec3::one() })
    };

    // Tests tmin
    let intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 0.0, z: -2.0 }, Vec3 { x: 0.0, y: 0.0, z: 1.0 });
    let mut non_intersection = sphere.intersects(&intersecting_ray, 1000.0, 10000.0);
    assert!(non_intersection.is_none());

    // Tests tmax
    non_intersection = sphere.intersects(&intersecting_ray, 0.0, 0.0001);
    assert!(non_intersection.is_none());
}
