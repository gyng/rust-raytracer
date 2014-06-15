pub use self::prim::Prim;

pub mod prim;

pub mod Prims {
    pub use self::sphere::Sphere;
    pub use self::plane::Plane;

    mod sphere;
    mod plane;
}
