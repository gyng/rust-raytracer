use std::num::Float;
use std::slice::Iter;
use geometry::bbox::get_bounds_from_objects;
use geometry::{BBox, Prim};
use raytracer::Ray;
use vec3::Vec3;

pub struct Octree<T> {
    prims: Vec<T>,
    infinites: Vec<T>, // for infinite prims (planes)
    root: OctreeNode<T>,
}

pub struct OctreeNode<T> {
    bbox: BBox,
    depth: i32,
    children: Vec<OctreeNode<T>>,
    leaf_data: Vec<OctreeData>,
}

#[derive(Clone, Copy)]
struct OctreeData {
    pub bbox: BBox,
    pub index: usize
}


impl OctreeData {
    pub fn intersects(&self, ray: &Ray) -> bool {
        self.bbox.intersects(ray)
    }
}


impl<T> OctreeNode<T> {
    #[allow(dead_code)]
    pub fn new(bbox: BBox, depth: i32) -> OctreeNode<T> {
        OctreeNode {
            bbox: bbox,
            depth: depth,
            children: Vec::new(),
            leaf_data: Vec::new(),
        }
    }

    fn subdivide(&mut self) {
        for x in range(0is, 2is) {
            for y in range(0is, 2is) {
                for z in range(0is, 2is) {
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

                    self.children.push(OctreeNode::new(child_bbox, self.depth - 1));
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, index: usize, object_bbox: BBox) -> () {
        // Max depth
        if self.depth <= 0 {
            self.leaf_data.push(OctreeData { index: index, bbox: object_bbox });
            return;
        }

        // Empty leaf node
        if self.is_leaf() && self.leaf_data.len() == 0 {
            self.leaf_data.push(OctreeData { index: index, bbox: object_bbox });
            return;
        }

        // Occupied leaf node and not max depth: subdivide node
        if self.is_leaf() && self.leaf_data.len() == 1 {
            self.subdivide();
            let old = self.leaf_data.remove(0);
            // Reinsert old node and then fall through to insert current object
            self.insert(old.index, old.bbox);
        }

        // Interior node (has children)
        for child in self.children.iter_mut() {
            if child.bbox.overlaps(&object_bbox) {
                child.insert(index, object_bbox);
            }
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
        let depth = (1.2 * (finites.len() as f64).log(8.0)).round() as i32;

        println!("Octree maximum depth {}", depth);
        let mut root_node = OctreeNode::new(bounds, depth);
        for (i, prim) in finites.iter().enumerate() {
            root_node.insert(i, prim.bounding().unwrap());
        }

        Octree {
            prims: finites,
            infinites: infinites,
            root: root_node,
        }
    }

    pub fn get_intersected_objects<'a>(&'a self, ray: &'a Ray) -> OctreeIterator<'a, Box<Prim+Send+Sync>> {
        OctreeIterator::new(self, ray)
    }
}


struct OctreeIterator<'a, T:'a> {
    prims: &'a [T],
    stack: Vec<&'a OctreeNode<T>>,
    cur_iter: Option<Iter<'a, OctreeData>>,
    ray: &'a Ray,
    infinites: Iter<'a, T>,
    just_infinites: bool
}


impl<'a> OctreeIterator<'a, Box<Prim+Send+Sync>> {
    fn new<'b>(octree: &'b Octree<Box<Prim+Send+Sync>>, ray: &'b Ray) -> OctreeIterator<'b, Box<Prim+Send+Sync>> {
            OctreeIterator {
            prims: &octree.prims[],
            stack: vec![&octree.root],
            cur_iter: None,
            ray: ray,
            infinites: octree.infinites.iter(),
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
            let (new_cur_iter, val) = match self.cur_iter.take() {
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
                        (Some(node.leaf_data.iter()), None)
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
