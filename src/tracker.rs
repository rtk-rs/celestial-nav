//! Star tracker

use geo::Coord;
use hifitime::Epoch;

pub struct Star {
    // (x, y) coordinates within the video frame (fraction of X)
    pub xy: Coord<f64>,
    // radius
    pub r: f64,
    // Epoch of new snapshot
    pub t: Epoch,
    // past xy
    pub past_xy: Coord<f64>,
    // past t
    pub past_t: Epoch,
}

/// Star [Tracker] designed to track N coordinates at the same time
pub struct Tracker<const N: usize> {
    pub stars: [Star; N],
}

impl<const N: usize> Tracker<N> {
    pub fn update(&mut self, xy: Coord, t: Epoch) {
    }
}