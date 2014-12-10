use std::cmp::{Eq, PartialOrd, Ord, Ordering};
use raytracer::Photon;

// For use in PhotonCache's querying of nearest N-neighbours
// #[deriving(Eq, PartialOrd, PartialEq)]
#[deriving(Clone)]
pub struct PhotonQuery {
    pub photon: Photon,
    pub distance_to_point: f64
}

// Need total ordering for binary heap
impl Ord for PhotonQuery {
    fn cmp(&self, other: &PhotonQuery) -> Ordering {
        if self.distance_to_point < other.distance_to_point { Less }
        else if self.distance_to_point == other.distance_to_point { Equal }
        else { Greater }
    }
}

impl Eq for PhotonQuery {
}

impl PartialOrd for PhotonQuery {
    fn partial_cmp(&self, other: &PhotonQuery) -> Option<Ordering> {
        Some(
            if self.distance_to_point < other.distance_to_point { Less }
            else if self.distance_to_point == other.distance_to_point { Equal }
            else { Greater }
        )
    }
}

impl PartialEq for PhotonQuery {
    fn eq(&self, other: &PhotonQuery) -> bool {
        self == other
    }
}
