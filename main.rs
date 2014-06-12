use vec3::Vec3;
mod vec3;
mod ray;
mod camera;
mod prim;
mod material;
mod intersection;
mod sphere;

fn main() {
    let a = Vec3 {x: 1.0, y: 1.0, z: 1.0};
    let b = Vec3 {x: 2.0, y: 3.0, z: 4.0};
    let result = a.dot(&b);
    println!("{}", result);
    println!("Length {}", a.len());
    // println!("{} {} {}", result.x, result.y, result.z);
}
