//! Basic video frame definitions
// use crate::Error;

// use itertools::{IntoChunks, Itertools};

// use core::slice::{Iter, IterMut};

use geo::Coord;

pub mod component;
use component::UnderlyingComponent;


// private modules
mod bitmap;

pub use bitmap::{BitMap, BitMapIter};

/// [Frame] describes a Video Frame of maximal (X, Y) dimension
pub struct Frame<'a, const XY: usize> {
    x: usize,
    y: usize,
    bitmap: BitMap<'a, XY>,
}


impl<'a, const XY: usize> Frame<'a, XY> {
    /// Create a new video [Frame]
    /// ## Input
    /// - x: frame width (in pixels)
    /// - y: frame height (in pixels)
    /// - bitmap: array of x*y dimension capture by video source.
    pub fn new(x: usize, y: usize, bitmap: BitMap<'a, XY>) -> Self {
        Self { x, y, bitmap }
    }

    /// Obtain reference to underlying video component, expressed as [UnderlyingPixel].
    pub(crate) fn get(&self, x: u16, y: u16) -> Option<&UnderlyingComponent> {
        self.bitmap.get(x, y)
    }

    // /// Obtain mutable reference to underlying video component, expressed as [UnderlyingPixel].
    // pub(crate) fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut UnderlyingPixel> {
    //     if y * self.x + x > XY {
    //         None
    //     } else {
    //         self.pixels.get_mut(x + y * self.x)
    //     }
    // }

    /// Returns total area of this [Frame]
    pub fn area(&self) -> usize {
        self.x * self.y
    }

    // /// Converts [Frame] to gray8
    // pub(crate) fn to_gray8(&mut self) -> Frame<XY> {
    //     Frame {
    //         x: self.x,
    //         y: self.y,
    //         pixels: {
    //             let mut pixels = self.pixels.clone();
    //             for p in pixels.iter_mut() {
    //                 *p = match p {
    //                     UnderlyingPixel::Gray8(gray8) => UnderlyingPixel::Gray8(*gray8),
    //                     UnderlyingPixel::Rgb8(rgb8) => UnderlyingPixel::Gray8(rgb8.to_gray8()),
    //                     UnderlyingPixel::YCbCr8(ycbcr8) => {
    //                         UnderlyingPixel::Gray8(ycbcr8.to_rgb8().to_gray8())
    //                     }
    //                 };
    //             }
    //             pixels
    //         },
    //     }
    // }

    // /// Converts [Frame] to gray8
    // pub(crate) fn to_gray8_mut(&mut self) {
    //     for pxl in self.pixels.iter_mut() {
    //         *pxl = match pxl {
    //             UnderlyingPixel::Gray8(gray8) => UnderlyingPixel::Gray8(*gray8),
    //             UnderlyingPixel::Rgb8(rgb8) => UnderlyingPixel::Gray8(rgb8.to_gray8()),
    //             UnderlyingPixel::YCbCr8(ycbcr8) => {
    //                 UnderlyingPixel::Gray8(ycbcr8.to_rgb8().to_gray8())
    //             }
    //         };
    //     }
    // }

    // /// Obtain a [FrameIter] from [Frame]
    // pub(crate) fn iter(&self) -> () {
    //     FrameIter {
    //         pixels: &self.pixels,
    //     }
    // }

    // /// Obtain a [FrameMutIter] from a mutable [Frame]
    // pub(crate) fn iter_mut(&'a mut self) -> FrameMutIter<'a, XY> {
    //     FrameMutIter {
    //         pixels: mut self.pixels,
    //     }
    // }

    // /// Obtain a line iterator from [Frame]
    // pub(crate) fn line_iter(&self) -> IntoChunks<Iter<'_, UnderlyingPixel>> {
    //     self.pixels.iter().chunks(self.x)
    // }

    // /// Obtain a mutable line iterator from [Frame]
    // pub(crate) fn line_iter_mut(&mut self) -> IntoChunks<IterMut<'_, UnderlyingPixel>> {
    //     self.pixels.iter_mut().chunks(self.x)
    // }

    // /// Computes the minimal value in gray scale
    // pub(crate) fn gray_scale_min(&self) -> u8 {
    //     let mut min = u8::MAX;
    //     for p in self.iter() {
    //         let gray = p.to_gray8();
    //         if gray < min {
    //             min = gray;
    //         }
    //     }
    //     min
    // }

    // /// Computes the maximal value in gray scale
    // pub(crate) fn gray_scale_max(&self) -> u8 {
    //     let mut max = 0;
    //     for p in self.iter() {
    //         let gray = p.to_gray8();
    //         if gray > max {
    //             max = gray;
    //         }
    //     }
    //     max
    // }

    /// Computes the maximal value in gray scale
    fn gray_scale_mean(&self) -> f64 {
        let mut acc = 0.0_f64;
        for p in self.bitmap.iter() {
            acc += p.to_gray8() as f64;
        }
        acc / XY as f64
    }

    /// Compute std deviation of the gray scale
    fn gray_scale_std_dev(&self, mean: f64) -> f64 {
        let mut acc = 0.0;
        for p in self.bitmap.iter() {
            acc += (p.to_gray8() as f64 - mean).powi(2);
        }
        (acc / XY as f64).sqrt()
    }

    /// Compute star luminosity threshold for this [Frame]
    pub(crate) fn gray8_star_luminosity_threshold(&self) -> u8 {
        let mean = self.gray_scale_mean();
        let stddev = self.gray_scale_std_dev(mean);
        (mean + 5.0 * stddev).round().clamp(0.0, 255.0) as u8
    }

    /// Estimates central coordinates simple histogram on gray scaled [Frame]
    pub fn star_coordinates_finder<const N: usize>(&self) -> [Coord<usize>; N] {
        let mut coords = [Coord::zero(); N];

        let threshold = self.gray8_star_luminosity_threshold();

        // // overwrite image by '1' where g8_(i,j) >= threshold:
        // for p in self.pixels.iter_mut() {
        //     let gray = p.to_gray8();
        //     if gray >= threshold {
        //         *p = UnderlyingPixel::Gray8(u8::MAX);
        //     } else {
        //         *p = UnderlyingPixel::Gray8(0);
        //     }
        // }

        coords
    }
}
