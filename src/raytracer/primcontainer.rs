use geometry::Prim;
use raytracer::Ray;

pub trait PrimContainer {
    fn get_intersection_objects<'a>(&'a self, ray: &'a Ray) -> Vec<&'a Box<Prim+Send+Sync>>;
}

#[allow(dead_code)]
pub struct VecPrimContainer {
    vec: Vec<Box<Prim+Send+Sync>>,
}

impl VecPrimContainer {
    #[allow(dead_code)]
    pub fn new(prims: Vec<Box<Prim+Send+Sync>>) -> VecPrimContainer {
        VecPrimContainer { vec: prims }
    }
}

impl PrimContainer for VecPrimContainer {
    // VecPrimContainer is dumb, so we just return the whole thing.
    // We could try pre-filtering here too though.
    fn get_intersection_objects<'a>(&'a self, _: &'a Ray) -> Vec<&'a Box<Prim+Send+Sync>> {
        let mut out: Vec<&Box<Prim+Send+Sync>> = Vec::new();
        for prim in self.vec.iter() {
            out.push(prim);
        }
        out
    }
}
