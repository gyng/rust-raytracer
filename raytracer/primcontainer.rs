use geometry::Prim;
use raytracer::Ray;


pub trait PrimContainer {
    fn get_intersection_objects<'a>(&'a self, ray: &'a Ray) -> Vec<&'a Box<Prim+Send+Share>>;
}


pub struct VecPrimContainer {
    vec: Vec<Box<Prim+Send+Share>>,
}

impl VecPrimContainer {
    pub fn new(prims: Vec<Box<Prim+Send+Share>>) -> VecPrimContainer {
        VecPrimContainer { vec: prims }
    }
}

impl PrimContainer for VecPrimContainer {
    // VecPrimContainer is dumb, so we just return the whole thing.
    // We could try pre-filtering here too though.
    fn get_intersection_objects<'a>(&'a self, _: &'a Ray) -> Vec<&'a Box<Prim+Send+Share>> {
        let mut out: Vec<&Box<Prim+Send+Share>> = Vec::new();
        for prim in self.vec.iter() {
            out.push(prim);
        }
        out
    }
}