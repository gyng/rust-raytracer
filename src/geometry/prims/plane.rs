use geometry::{BBox, Prim};
use material::Material;
use raytracer::{Ray, Intersection};
use vec3::Vec3;

#[cfg(test)]
use material::materials::FlatMaterial;

#[allow(dead_code)]
pub struct Plane {
    pub a: f64, // normal.x
    pub b: f64, // normal.y
    pub c: f64, // normal.z
    pub d: f64,
    pub material: Box<Material+Send+Share>
}

impl Prim for Plane {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>> {
        let n = Vec3 { x: self.a, y: self.b, z: self.c };
        let nrd = n.dot(&ray.direction);
        let nro = n.dot(&ray.origin);
        let t = (-self.d - nro) / nrd;

        if t < t_min || t > t_max {
            None
        } else {
            let intersection_point = ray.origin + ray.direction.scale(t);
            let u_axis = Vec3 { x: n.y, y: n.z, z: -n.x };
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

#[test]
fn it_intersects() {
    let plane = Plane { a: 0.0, b: 1.0, c: 0.0, d: 0.0, material: box FlatMaterial { color: Vec3::one() } };

    // Tests actual intersection
    let intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 1.0, z: 0.0 }, Vec3 { x: 0.0, y: -1.0, z: 0.0 });
    let intersection = plane.intersects(&intersecting_ray, 0.0, 10.0).unwrap();
    assert_eq!(intersection.position.x, 0.0);
    assert_eq!(intersection.position.y, 0.0);
    assert_eq!(intersection.position.z, 0.0);
    assert_eq!(intersection.n.x, 0.0);
    assert_eq!(intersection.n.y, 1.0);
    assert_eq!(intersection.n.z, 0.0);

    // Parallel ray
    let mut non_intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 1.0, z: 0.0 }, Vec3 { x: 1.0, y: 0.0, z: 1.0 });
    let mut non_intersection = plane.intersects(&non_intersecting_ray, 0.0, 10000.0);
    assert!(non_intersection.is_none());

    // Ray in opposite direction
    non_intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 1.0, z: 0.0 }, Vec3 { x: 0.0, y: 1.0, z: 0.0 });
    non_intersection = plane.intersects(&non_intersecting_ray, 0.0, 10.0);
    assert!(non_intersection.is_none());
}

#[test]
fn it_intersects_only_in_tmin_tmax() {
    let plane = Plane { a: 0.0, b: 1.0, c: 0.0, d: 0.0, material: box FlatMaterial { color: Vec3::one() } };

    // Tests tmin
    let intersecting_ray = Ray::new(Vec3 { x: 0.0, y: 1.0, z: 0.0 }, Vec3 { x: 0.0, y: -1.0, z: 0.0 });
    let mut non_intersection = plane.intersects(&intersecting_ray, 1000.0, 10000.0);
    assert!(non_intersection.is_none());

    // Tests tmax
    non_intersection = plane.intersects(&intersecting_ray, 0.0, 0.0001);
    assert!(non_intersection.is_none());
}
