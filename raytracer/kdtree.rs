// use std::slice::Items;
// use geometry::bbox::get_bounds_from_objects;
use geometry::BBox;
use geometry::bbox::get_bounds_from_photons;
use raytracer::{Photon};

// #[cfg(test)]
use vec3::Vec3;


#[deriving(Clone)]
pub struct KDNode {
    pub photon: Photon,
    pub bbox: BBox,
    pub left_child: Option<Box<KDNode>>,
    pub right_child: Option<Box<KDNode>>,
    pub axis: uint
}


impl KDNode {
    pub fn query_region(current: &KDNode, target: BBox) -> Vec<Photon> {
        let mut results: Vec<Photon> = Vec::new();

        if !target.overlaps(&current.bbox) {
            return results;
        }

        if target.inside(&current.photon.position) {
            results.push(current.photon);
        }

        match current.left_child {
            Some(box ref child) => {
                if target.overlaps(&child.bbox) {
                    results = results + KDNode::query_region(child, target);
                }
            },
            None => {}
        }

        match current.right_child {
            Some(box ref child) => {
                if target.overlaps(&child.bbox) {
                    results = results + KDNode::query_region(child, target);
                }
            },
            None => {}
        }

        results
    }

    // Need to upgrade this to do n-nearest neighbours
    pub fn nearest_neighbour(current: &Option<Box<KDNode>>, target: Vec3, best: Option<Photon>) -> Option<Photon> {
        match *current {
            None => return best,
            Some(ref current) => {
                let mut new_best = match best {
                    Some(b) => Some(b),
                    None => Some(current.photon)
                };

                match new_best {
                    Some(b) => {
                        if (target - current.photon.position).len() < (target - b.position).len() {
                            new_best = Some(current.photon);
                        }
                    },
                    None => fail!("This should not happen")
                }

                match new_best {
                    Some(b) => new_best = KDNode::nearest_neighbour(current.nearer_child(target), target, new_best),
                    None => fail!("This should not happen")
                }

                // Can be optimised (see SO link above)
                match new_best {
                    Some(b) => new_best = KDNode::nearest_neighbour(current.away_child(target), target, new_best),
                    None => fail!("This should not happen")
                }

                new_best
            }
        }
    }

    fn nearer_child(&self, point: Vec3) -> &Option<Box<KDNode>> {
        match self.axis {
            0 => if self.photon.position.x < point.x { &self.left_child } else { &self.right_child },
            1 => if self.photon.position.y < point.y { &self.left_child } else { &self.right_child },
            2 => if self.photon.position.z < point.z { &self.left_child } else { &self.right_child },
            _ => fail!("Only 3D supported")
        }
    }

    fn away_child(&self, point: Vec3) -> &Option<Box<KDNode>> {
        match self.axis {
            0 => if self.photon.position.x > point.x { &self.left_child } else { &self.right_child },
            1 => if self.photon.position.y > point.y { &self.left_child } else { &self.right_child },
            2 => if self.photon.position.z > point.z { &self.left_child } else { &self.right_child },
            _ => fail!("Only 3D supported")
        }
    }

    #[allow(dead_code)]
    pub fn is_leaf(&self) -> bool {
        match self.left_child {
            Some(_) => match self.right_child {
                Some(_) => false,
                None => false
            },
            None => {match self.right_child {
                Some(_) => false,
                None => true
            }}
        }
    }
}


pub struct KDTree;


// TODO: Generalise this for Prims as well
impl KDTree {
    #[allow(dead_code)]
    pub fn new_from_photons(point_list: Vec<Photon>, depth: int) -> Option<KDNode> {
        if point_list.len() == 0 {
            return None;
        }

        // Dimension of element (3 for a Vec3)
        // let k = point_list[0].position.len();
        let k = 3;

        // Cycle through axes as we go deeper
        let axis = depth % k;

        let mut sorted_point_list = point_list.clone();
        sorted_point_list.sort_by(|a, b| {
            let cmp = match axis {
                0 => a.position.x.partial_cmp(&b.position.x),
                1 => a.position.y.partial_cmp(&b.position.y),
                2 => a.position.z.partial_cmp(&b.position.z),
                _ => fail!("KDTree is 3D-only (for now)")
            };

            match cmp {
                Some(x) => x,
                None => fail!("Could not compare values in KDTree")
            }
        });

        let median_index = sorted_point_list.len() / 2;

        let left_child = KDTree::new_from_photons(
                Vec::from_slice(sorted_point_list.slice_to(median_index)),
                depth + 1);
        let right_child = KDTree::new_from_photons(
                Vec::from_slice(sorted_point_list.slice_from(median_index + 1)),
                depth + 1);

        Some(KDNode {
            photon: sorted_point_list[median_index],
            bbox: get_bounds_from_photons(&point_list),
            left_child: match left_child { Some(c) => {Some(box c)}, None => None },
            right_child: match right_child { Some(c) => {Some(box c)}, None => None },
            axis: axis as uint
        })
    }
}


#[test]
fn it_creates_and_range_queries() {
    let mut photons: Vec<Photon> = Vec::new();
    photons.push(Photon {
        position: Vec3 {x: 0.0, y: 0.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 1.0, y: 0.0, z: 0.0}
    });
    photons.push(Photon {
        position: Vec3 {x: 0.5, y: -0.5, z: 0.5},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 1.1, y: 0.0, z: 0.0}
    });

    // Not in region
    photons.push(Photon {
        position: Vec3 {x: 2.0, y: 0.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 0.0, y: 0.0, z: 0.0}
    });
    photons.push(Photon {
        position: Vec3 {x: 0.0, y: 2.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 0.0, y: 0.0, z: 0.0}
    });
    photons.push(Photon {
        position: Vec3 {x: 0.0, y: 0.0, z: -2.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 0.0, y: 0.0, z: 0.0}
    });

    let tree = match KDTree::new_from_photons(photons, 0) {
        Some(x) => x,
        None => fail!("Could not create KD-Tree")
    };

    let target = BBox {
        min: Vec3 {x: -1.0, y:-1.0, z: -1.0},
        max: Vec3 {x: 1.0, y: 1.0, z: 1.0}
    };

    let results = KDNode::query_region(&tree, target);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].power.x, 1.0);
    assert_eq!(results[1].power.x, 1.1);
}

#[test]
fn it_creates_and_gets_nearest_neighbour() {
    let mut photons: Vec<Photon> = Vec::new();
    photons.push(Photon {
        position: Vec3 {x: 0.0, y: 0.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 1.0, y: 0.0, z: 0.0}
    });
    photons.push(Photon {
        position: Vec3 {x: 0.5, y: -0.5, z: 0.5},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 1.1, y: 0.0, z: 0.0}
    });
    photons.push(Photon {
        position: Vec3 {x: 2.0, y: 0.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 8.8, y: 0.0, z: 0.0}
    });
    photons.push(Photon {
        position: Vec3 {x: 0.0, y: 2.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 0.0, y: 0.0, z: 0.0}
    });
    photons.push(Photon {
        position: Vec3 {x: 0.0, y: 0.0, z: -2.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 0.0, y: 0.0, z: 0.0}
    });

    let tree = match KDTree::new_from_photons(photons, 0) {
        Some(x) => x,
        None => fail!("Could not create KD-Tree")
    };

    let target = Vec3 {x: 0.4, y: 0.1, z: -0.1};
    let result = match KDNode::nearest_neighbour(Some(&tree), target, None) {
        Some(r) => r,
        None => fail!("No photonic neighbour found!?")
    };
    assert_eq!(result.power.x, 1.0);

    let target2 = Vec3{x: 2.1, y: 0.0, z: 0.2};
    let result2 = match KDNode::nearest_neighbour(Some(&tree), target2, None) {
        Some(r) => r,
        None => fail!("No photonic neighbour found!?")
    };
    assert_eq!(result2.power.x, 8.8);
}
