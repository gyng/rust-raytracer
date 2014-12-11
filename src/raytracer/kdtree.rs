// use std::slice::Items;
// use geometry::bbox::get_bounds_from_objects;
use geometry::BBox;
use geometry::bbox::get_bounds_from_photons;
// use geometry::bbox;
// use geometry::bbox::get_bounds_from_photons;
use raytracer::{Photon, PhotonQuery};
use vec3::Vec3;
use std::collections::BinaryHeap;


#[deriving(Clone)]
pub struct KDNode {
    pub photon: Photon,
    pub bbox: BBox,
    pub left_child: Option<Box<KDNode>>,
    pub right_child: Option<Box<KDNode>>
}


impl KDNode {
    pub fn query_region(current: Box<KDNode>, target: BBox) -> Vec<Photon> {
        let mut results: Vec<Photon> = Vec::new();

        if !target.overlaps(&current.bbox) {
            return results;
        }

        if target.inside(&current.photon.position) {
            results.push(current.photon);
        }

        match current.left_child {
            Some(ref child) => {
                if target.overlaps(&child.bbox) {
                    results = results + KDNode::query_region(child.clone(), target);
                }
            },
            None => {}
        }

        match current.right_child {
            Some(ref child) => {
                if target.overlaps(&child.bbox) {
                    results = results + KDNode::query_region(child.clone(), target);
                }
            },
            None => {}
        }

        results
    }

    // Adapted from Realistic Image Synthesis using Photon Mapping (Henrik Wann Jensen) pp. 73
    pub fn query_nearest(results: &mut BinaryHeap<PhotonQuery>, current: Box<KDNode>, target: Vec3, max_dist: f64, max_photons: uint) -> () {
        let delta = (current.bbox.center() - target).len();

        let (first_child, second_child) = if delta > 0.0 {
            (current.left_child.clone(), current.right_child.clone())
        } else {
            (current.right_child.clone(), current.left_child.clone())
        };

        match first_child {
            Some(child) => {
                KDNode::query_nearest(results, child, target, max_dist, max_photons);
            },
            None => {}
        }

        // if delta * delta < max_dist * max_dist {
            match second_child {
                Some(child) => {
                    KDNode::query_nearest(results, child, target, max_dist, max_photons);
                },
                None => {}
            }
        // }

        let photon_dist = (current.photon.position - target).len();

        if photon_dist < max_dist {
            if results.len() < max_photons {
                results.push(PhotonQuery { distance_to_point: photon_dist, photon: current.photon.clone() });
            } else {
                if let Some(farthest) = results.top().cloned() {
                    if photon_dist < farthest.distance_to_point {
                        results.replace(PhotonQuery { distance_to_point: photon_dist, photon: current.photon.clone() });
                    }
                }
            }
        }
    }
}


pub struct KDTree;


// TODO: Generalise this for Prims as well
impl KDTree {
    pub fn new_from_photons(point_list: Vec<Photon>, depth: int) -> Option<Box<KDNode>> {
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
                _ => panic!("KDTree is 3D-only (for now)")
            };

            match cmp {
                Some(x) => x,
                None => panic!("Could not compare values in KDTree")
            }
        });

        let median_index = sorted_point_list.len() / 2;

        let left_child = KDTree::new_from_photons(
                sorted_point_list.slice_to(median_index).to_vec(),
                depth + 1);
        let right_child = KDTree::new_from_photons(
                sorted_point_list.slice_from(median_index + 1).to_vec(),
                depth + 1);

        Some(box KDNode {
            photon: sorted_point_list[median_index],
            bbox: get_bounds_from_photons(&point_list),
            left_child: left_child,
            right_child: right_child
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
        None => panic!("Could not create KD-Tree")
    };

    let target = BBox {
        min: Vec3 {x: -1.0, y:-1.0, z: -1.0},
        max: Vec3 {x: 1.0, y: 1.0, z: 1.0}
    };

    let results = KDNode::query_region(tree, target);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].power.x, 1.0);
    assert_eq!(results[1].power.x, 1.1);
}

#[test]
fn it_creates_and_gets_nearest_photons() {
    let mut photons: Vec<Photon> = Vec::new();
    photons.push(Photon {
        position: Vec3 {x: 0.0, y: 0.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 0.2, y: 0.0, z: 0.0}
    });
    photons.push(Photon {
        position: Vec3 {x: 0.5, y: 0.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 0.1, y: 0.0, z: 0.0}
    });

    // Not in region
    photons.push(Photon {
        position: Vec3 {x: 2.0, y: 0.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 0.0, y: 0.0, z: 0.0}
    });
    photons.push(Photon {
        position: Vec3 {x: 5.0, y: 0.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 0.0, y: 0.0, z: 0.0}
    });
    photons.push(Photon {
        position: Vec3 {x: 10.0, y: 0.0, z: 0.0},
        incoming_dir: Vec3::zero(),
        power: Vec3 {x: 0.0, y: 0.0, z: 0.0}
    });

    let tree = match KDTree::new_from_photons(photons, 0) {
        Some(x) => x,
        None => panic!("Could not create KD-Tree")
    };

    let mut nearby_photons: BinaryHeap<PhotonQuery> = BinaryHeap::with_capacity(2 + 1);
    let target = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let max_dist = 5.0;
    let max_photons = 2;
    KDNode::query_nearest(&mut nearby_photons, tree, target, max_dist, max_photons);

    assert_eq!(nearby_photons.len(), 2);
    assert_eq!(nearby_photons.top().unwrap().distance_to_point, 0.5);
    assert_eq!(nearby_photons.pop().unwrap().photon.power.x, 0.1);
    assert_eq!(nearby_photons.pop().unwrap().photon.power.x, 0.2);
}
