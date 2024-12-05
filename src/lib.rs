#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

/*
 * Celestial-NAV is part of the RTK-RS framework.
 * Copyright (C) 2022-onwards Guillaume W. Bres <guillaume.bressaix@gmail.com> et al. 
 * (cf. https://github.com/rtk-rs/celestial-nav/graphs/contributors)
 * The RTK-RS framework is shipped under the MPL-2.0 License: https://www.mozilla.org/en-US/MPL/2.0
 *
 * Documentation: https://github.com/rtk-rs/celestial-nav
 */


#[cfg(feature = "std")]
extern crate std;

pub mod frame;
pub mod solver;

pub(crate) mod tracker;

use frame::Frame;

pub mod prelude {
    pub use crate::frame::{Frame, BitMap};
    pub use crate::solver::Solver;
    pub use nalgebra::{Rotation3, Matrix3};
    pub use hifitime::Epoch;
}

/// Implement the [VideoSource] Trait to provide new 
/// Video [Frame]s to the solver.
/// Generics:
/// - XY: maximal total video frame size (in pixels)
/// to ever be published. For memory allocation purposes.
pub trait VideoSource<const XY: usize> {
    fn next(&mut self) -> Option<Frame<XY>>;
}

pub enum Error {
    /// Internal error due to bad video format
    /// considerations. Should never happen!
    VideoFormatError,
    /// Internal error due to bad dimensions.
    /// Should never happen!
    VideoDimensionError,
}
