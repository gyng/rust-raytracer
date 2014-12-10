// use std::slice::Items;
// use geometry::bbox::get_bounds_from_objects;
use geometry::BBox;
use geometry::bbox::get_bounds_from_photons;
// use geometry::bbox;
// use geometry::bbox::get_bounds_from_photons;
use raytracer::{Photon};
use vec3::Vec3;


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

        // println!("At current [{}, {}, {}]",
        //     current.photon.position.x,
        //     current.photon.position.y,
        //     current.photon.position.z,
        // );

        if !target.overlaps(&current.bbox) {
            return results;
        }

        if target.inside(&current.photon.position) {
            // println!("\t\t ADD is in target");
            results.push(current.photon);
        }

        match current.left_child {
            Some(ref child) => {
                // println!("\t left child");
                if target.overlaps(&child.bbox) {
                    // println!("\t\t is in target");
                    results = results + KDNode::query_region(child.clone(), target);
                }
            },
            None => {/*println!("\t no left child");*/}
        }

        match current.right_child {
            Some(ref child) => {
                // println!("\t right child");
                if target.overlaps(&child.bbox) {
                    // println!("\t\t is in target");
                    results = results + KDNode::query_region(child.clone(), target);
                }
            },
            None => {/*println!("\t no right_child");*/}
        }

        results
    }

    #[allow(dead_code)]
    pub fn is_leaf(&self) -> bool {
        match self.left_child {
            Some(_) => match self.right_child {
                Some(_) => {/*println!("a");*/ false},
                None => {/*println!("b");*/ false}
            },
            None => {match self.right_child {
                Some(_) => {/*println!("c");*/ false},
                None => {/*println!("d");*/ true}
            }}
        }
    }
}


pub struct KDTree;


// TODO: Generalise this for Prims as well
impl KDTree {
    #[allow(dead_code)]
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
