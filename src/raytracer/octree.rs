use std::num::Float;
use std::slice::Iter;
use geometry::bbox::get_bounds_from_objects;
use geometry::{BBox, Prim};
use raytracer::Ray;
use vec3::Vec3;

pub struct Octree<T> {
    pub prims: Option<Vec<T>>,
    pub bbox: BBox,
    pub depth: int,
    pub children: Vec<Octree<T>>,
    pub data: Vec<OctreeData>,
    pub infinites: Vec<T> // for infinite prims (planes)
}

#[derive(Clone, Copy)]
struct OctreeData {
    pub bbox: Option<BBox>,
    pub index: uint
}


impl OctreeData {
    pub fn intersects(&self, ray: &Ray) -> bool {
        match self.bbox {
            Some(bbox) => bbox.intersects(ray),
            None => true
        }
    }
}


impl<T> Octree<T> {
    #[allow(dead_code)]
    pub fn new(bbox: BBox, depth: int) -> Octree<T> {
        let vec_children: Vec<Octree<T>> = Vec::new();
        let vec_data: Vec<OctreeData> = Vec::new();
        let vec_infinite_data: Vec<T> = Vec::new();

        Octree {
            prims: None,
            bbox: bbox,
            depth: depth,
            children: vec_children,
            data: vec_data,
            infinites: vec_infinite_data
        }
    }

    fn subdivide(&mut self) {
        for x in range(0i, 2i) {
            for y in range(0i, 2i) {
                for z in range(0i, 2i) {
                    let len = self.bbox.len();

                    let child_bbox = BBox {
                        min: Vec3 {
                            x: self.bbox.min.x + x as f64 * len.x / 2.0,
                            y: self.bbox.min.y + y as f64 * len.y / 2.0,
                            z: self.bbox.min.z + z as f64 * len.z / 2.0
                        },
                        max: Vec3 {
                            x: self.bbox.max.x - (1 - x) as f64 * len.x / 2.0,
                            y: self.bbox.max.y - (1 - y) as f64 * len.y / 2.0,
                            z: self.bbox.max.z - (1 - z) as f64 * len.z / 2.0,
                        }
                    };

                    self.children.push(Octree::new(child_bbox, self.depth - 1));
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, index: uint, object_bbox: Option<BBox>) -> () {
        match object_bbox {
            // Finite object
            Some(object_bbox) => {
                // Max depth
                if self.depth <= 0 {
                    self.data.push(OctreeData { index: index, bbox: Some(object_bbox) });
                    return;
                }

                // Empty leaf node
                if self.is_leaf() && self.data.len() == 0 {
                    self.data.push(OctreeData { index: index, bbox: Some(object_bbox) });
                    return;
                }

                // Occupied leaf node and not max depth: subdivide node
                if self.is_leaf() && self.data.len() == 1 {
                    self.subdivide();
                    let old = self.data.remove(0);
                    // Reinsert old node and then fall through to insert current object
                    self.insert(old.index, old.bbox);
                }

                // Interior node (has children)
                for child in self.children.iter_mut() {
                    if child.bbox.overlaps(&object_bbox) {
                        child.insert(index, Some(object_bbox));
                    }
                }
            }

            // Infinite object without bounds, this is added to
            // all get_intersection_indices calls
            None => panic!("Don't push infinites this way")
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
}

// TODO: sell: Prims can later implement a Bounded3D trait containing a method
//             that returns Option<BBox>.  Some if finite, None if infinite.
//             Then we can use impl<T: Bounded3D> Octree<Box<T>>, probably!

impl Octree<Box<Prim+Send+Sync>> {
    #[allow(dead_code)]
    pub fn new_from_prims(prims: Vec<Box<Prim+Send+Sync>>) -> Octree<Box<Prim+Send+Sync>> {
        let bounds = get_bounds_from_objects(&prims);
        let (finites, infinites): (Vec<Box<Prim+Send+Sync>>, Vec<Box<Prim+Send+Sync>>) = prims.into_iter().partition(|prim| prim.bounding().is_some());
        // pbrt recommended max depth for a k-d tree (though, we're using an octree)
        // For a k-d tree: 8 + 1.3 * log2(N)
        let depth = (1.2 * (finites.len() as f64).log(8.0)).round() as int;

        println!("Octree maximum depth {}", depth);
        let mut octree = Octree::new(bounds, depth);

        for (i, prim) in finites.iter().enumerate() {
            octree.insert(i, prim.bounding());
        }
        octree.prims = Some(finites);
        octree.infinites = infinites;

        octree
    }

    pub fn get_intersected_objects<'a>(&'a self, ray: &'a Ray) -> OctreeIterator<'a, Box<Prim+Send+Sync>> {
        OctreeIterator::new(self, ray)
    }
}


struct OctreeIterator<'a, T:'a> {
    prims: &'a Vec<T>,
    stack: Vec<&'a Octree<T>>,
    cur_iter: Option<Iter<'a, OctreeData>>,
    ray: &'a Ray,
    infinites: Iter<'a, T>,
    just_infinites: bool

}


impl<'a> OctreeIterator<'a, Box<Prim+Send+Sync>> {
    fn new<'b>(root: &'b Octree<Box<Prim+Send+Sync>>, ray: &'b Ray) -> OctreeIterator<'b, Box<Prim+Send+Sync>> {
        let prims = match root.prims {
            Some(ref prims) => prims,
            None => panic!("OctreeIterator must be constructed from an Octree root")
        };
        OctreeIterator {
            prims: prims,
            stack: vec![root],
            cur_iter: None,
            ray: ray,
            infinites: root.infinites.iter(),
            just_infinites: false
        }
    }
}


impl<'a> Iterator for OctreeIterator<'a, Box<Prim+Send+Sync>> {
    type Item = &'a Box<Prim+Send+Sync>;

    fn next(&mut self) -> Option<&'a Box<Prim+Send+Sync>> {
        if self.just_infinites {
            return self.infinites.next();
        }
        loop {
            let (new_cur_iter, val) = match self.cur_iter {
                Some(mut cur_iter) => match cur_iter.next() {
                    Some(val) => (Some(cur_iter), Some(val)),
                    None => (None, None)
                },
                None => match self.stack.pop() {
                    Some(node) => {
                        for child in node.children.iter() {
                            if child.bbox.intersects(self.ray) {
                                self.stack.push(child);
                            }
                        }
                        (Some(node.data.iter()), None)
                    },
                    None => break  // Empty stack and no iterator
                }
            };
            self.cur_iter = new_cur_iter;
            match val {
                Some(val) => {
                    if val.intersects(self.ray) {
                        return Some(&self.prims[val.index]);
                    }
                },
                None => (),
            }
        }
        self.just_infinites = true;
        self.infinites.next()
    }
}
