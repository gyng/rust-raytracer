use geometry::BBox;
use raytracer::Ray;
use vec3::Vec3;

pub struct Octree {
    pub bbox: BBox,
    pub depth: int,
    pub children: Vec<Octree>,
    pub data: Vec<OctreeData> // indices
}

#[deriving(Clone)]
struct OctreeData {
    pub bbox: BBox,
    pub index: uint
}

impl Octree {
    #[allow(dead_code)]
    pub fn new(bbox: BBox, depth: int) -> Octree {
        let vec_children: Vec<Octree> = Vec::new();
        let vec_data: Vec<OctreeData> = Vec::new();

        Octree {
            bbox: bbox,
            depth: depth,
            children: vec_children,
            data: vec_data
        }
    }

    #[allow(dead_code)]
    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    #[allow(dead_code)]
    pub fn subdivide(&mut self) -> () {
        for x in range(0, 2) {
            for y in range(0, 2) {
                for z in range(0, 2) {
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
    pub fn insert(&mut self, index: uint, object_bbox: BBox) -> () {
        // Max depth
        if self.depth <= 0 {
            self.data.push(OctreeData {index: index, bbox: object_bbox});
            return
        }

        // Empty leaf node
        if self.is_leaf() && self.data.len() == 0 {
            self.data.push(OctreeData {index: index, bbox: object_bbox});
            return
        }

        // Occupied leaf node and not max depth: subdivide node
        if self.is_leaf() && self.data.len() == 1 {
            self.subdivide();
            let old = match self.data.shift() {
                Some(x) => {x},
                None => {fail!("Trying to subdivide empty node in octree insertion")}
            };
            self.insert(old.index, old.bbox);
            return
        }

        // Interior node (has children)
        if !self.is_leaf() {
            for child in self.children.mut_iter() {
                if self.bbox.overlaps(&child.bbox) {
                    child.insert(index, object_bbox);
                }
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

            objects
        }
    }
}
