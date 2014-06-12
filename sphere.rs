use vec3::Vec3;
use ray::Ray;
use material::Material;
use prim::Prim;
use intersection::Intersection;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<Material>
}

impl Prim for Sphere {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Intersection<'a> {
        let i = ray.origin - self.center;
        let a = 1.0;
        let b = 2.0 * ray.direction.dot(&i);
        let c = i.dot(&i) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant <= 0.0 {
            Intersection {
                intersects: false,
                n: None,
                t: None,
                position: None,
                material: None
            }
        } else {
            // Up to two intersections
            let disc_sqrt = discriminant.sqrt();
            let t1 = (-b + disc_sqrt) / 2.0 * a;
            let t2 = (-b - disc_sqrt) / 2.0 * a;

            if t1 >= t_min && t1 <= t_max ||
               t2 >= t_min && t2 <= t_max {
                // Valid intersection(s): get nearer intersection
                let t = t1.abs().min(t2.abs());
                let intersection_point = ray.origin + ray.direction.scale(t);
                let n = (intersection_point - self.center).unit();

                Intersection {
                    intersects: true,
                    n: Some(n),
                    t: Some(t),
                    position: Some(intersection_point),
                    material: Some(&'a self.material)
                }
            } else {
                Intersection {
                    intersects: false,
                    n: None,
                    t: None,
                    position: None,
                    material: None
                }
            }
        }
    }
}
