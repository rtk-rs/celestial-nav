Celestial NAV
=============

[![crates.io](https://img.shields.io/crates/v/celestial-nav.svg)](https://crates.io/crates/celestial-nav)
[![crates.io](https://docs.rs/celestial-nav/badge.svg)](https://docs.rs/celestial-nav/badge.svg)

`Celestial-NAV` is a pure rust library that aims at providing celestial navigation by means
of stars image analysis. It is intended for work in close relationship with a navigation framework, 
an IMU sensor and a camera sensor.

## no-std

The library works without `std`. The idea is to augment a navigation framework
on an embedded platform.

## Input

`Celestial-NAV` works from two inputs:

- a frame picture of (X, Y) dimensions
- the rover attitude (or state vector)
- both need to be regularly updated

## Core algorithm

`Celestial-NAV` extracts the star `(x, y)` coordinates in the camera frame
and project them into 3D.


## Dependencies

`Celestial-nav` depends on the following key elements:

- `geo` for 2D coordinates
- `nalgebra`: solver
- `hifitime`: timing
