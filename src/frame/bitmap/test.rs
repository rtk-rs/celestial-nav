
use crate::tracker::Star;

use crate::frame::{BitMap, UnderlyingComponent};

use image::open as image_open;
use image::DynamicImage;
use image::EncodableLayout;

#[cfg(feature = "std")]
use std::path::Path;

#[cfg(feature = "std")]
pub struct OwnedBitMap {
    map: Vec<UnderlyingComponent>,
}

#[cfg(feature = "std")]
impl<'a, const XY: usize> BitMap<'a, XY> {
    
    // /// Helpers to build a test bitmap based on [Star] content
    // pub fn from_stars_coords_radius(stars: Vec<Star>) -> Self {
    //     let mut s = Self::from_vec(Vec::with_capacity(XY));
    //     s
    // }
}