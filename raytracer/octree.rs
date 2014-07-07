use geometry::bbox::get_bounds_from_objects;
use geometry::{BBox, Prim};
use raytracer::Ray;
use vec3::Vec3;

pub struct Octree {
    pub bbox: BBox,
    pub depth: int,
    pub children: Vec<Octree>,
    pub data: Vec<OctreeData>,
    pub infinite_data: Vec<OctreeData> // for infinite prims (planes)
}

#[deriving(Clone)]
struct OctreeData {
    pub bbox: Option<BBox>,
    pub index: uint
}

impl Octree {
    #[allow(dead_code)]
    pub fn new(bbox: BBox, depth: int) -> Octree {
        let vec_children: Vec<Octree> = Vec::new();
        let vec_data: Vec<OctreeData> = Vec::new();
        let vec_infinite_data: Vec<OctreeData> = Vec::new();

        Octree {
            bbox: bbox,
            depth: depth,
            children: vec_children,
            data: vec_data,
            infinite_data: vec_infinite_data
        }
    }

    #[allow(dead_code)]
    pub fn new_from_prims(prims: &Vec<Box<Prim+Send+Share>>) -> Octree {
        let bounds = get_bounds_from_objects(prims);
        // pbrt recommended max depth for a k-d tree (though, we're using an octree)
        // For a k-d tree: 8 + 1.3 * log2(N)
        let depth = (1.2 * (prims.len() as f64).log(8.0)).round() as int;
        println!("Octree maximum depth {}", depth);
        let mut octree = Octree::new(bounds, depth);

        for i in range(0, prims.len()) {
            octree.insert(i, prims.get(i).bounding());
        }

        octree
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    fn subdivide(&mut self) -> () {
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
                    self.data.push(OctreeData {index: index, bbox: Some(object_bbox)});
                    return;
                }

                // Empty leaf node
                if self.is_leaf() && self.data.len() == 0 {
                    self.data.push(OctreeData {index: index, bbox: Some(object_bbox)});
                    return;
                }

                // Occupied leaf node and not max depth: subdivide node
                if self.is_leaf() && self.data.len() == 1 {
                    self.subdivide();
                    let old = match self.data.shift() {
                        Some(x) => {x},
                        None => {fail!("Trying to subdivide empty node in octree insertion")}
                    };
                    // Reinsert old node and then fall through to insert current object
                    self.insert(old.index, old.bbox);
                }

                // Interior node (has children)
                for child in self.children.mut_iter() {
                    if child.bbox.contains(&object_bbox) {
                        child.insert(index, Some(object_bbox));
                    }
                }
            }

            // Infinite object without bounds, this is added to
            // all get_intersection_objects calls
            None => {
                self.infinite_data.push(OctreeData {index: index, bbox: None});
            }
        }

    }

    #[allow(dead_code)]
    pub fn get_intersection_objects(&self, ray: &Ray) -> Vec<OctreeData> {
        if self.is_leaf() {
            self.data.clone()
        } else {
            let mut objects: Vec<OctreeData> = Vec::new();

            for child in self.children.iter() {
                if child.bbox.intersects(ray) {
                    objects = objects.append(child.get_intersection_objects(ray).as_slice());
                }
            }

            objects.append(self.infinite_data.as_slice())
        }
    }
}
