pub use self::prim::Prim;
pub use self::mesh::Mesh;

pub mod prim;
pub mod mesh;

pub mod prims {
    pub use self::plane::Plane;
    pub use self::sphere::Sphere;
    pub use self::triangle::Triangle;

    mod plane;
    mod sphere;
    mod triangle;
}
