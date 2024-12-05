//! Celestian Navigation Solver

use crate::{
    prelude::{Epoch, Frame, Matrix3, Rotation3},
    VideoSource,
};

use nalgebra::{DMatrix, Matrix1x4};

use geo::Coord;
use crate::tracker::Tracker;

/// Internal Finite [State] Machine
#[derive(Debug, Default, Clone)]
enum State {
    #[default]
    /// Need to gather a new video [Frame] snapshot
    Capture,
    /// Process video [Frame] snapshot
    VideoProcessing,
    /// Video post processing, 3D calculation and projection
    PostProcessing,
}

/// [Solver] to run the Celestial navigation computation.
/// ## Generics:
/// - V: [VideoSource] implementation
/// - XY: [usize] maximal snapshot size to ever be supported.
/// For memory allocation purposes.
pub struct Solver<'a, const XY: usize, V: VideoSource<XY>> {
    /// [VideoSource] implementation
    video_src: V,
    /// Latest video [Frame] snapshot
    video_frame: Option<Frame<'a, XY>>,
    /// Internal state
    state: State,
    /// Fixed Body / Camera rotation matrix
    body_camera_rot3: Rotation3<f64>,
    /// Zenith angles of 4 stars in sight
    zeniths: Matrix1x4<f64>,
}

impl<'a, const XY: usize, V: VideoSource<XY>> Solver<'a, XY, V> {

    /// Builds a new [Solver].
    /// ## Inputs
    /// 
    /// - video_src: [VideoSource] is a [Frame] provider.
    /// 
    /// - body_camera: is the direct cosine rotation matrix,
    /// expressed as [Rotation3] that can transform the camera frame
    /// to the fixed body frame of the rover. Ideally you should calibrate
    /// this matrix and the camera mount should remain stable throughout navigation.
    pub fn new_fixed_body_camera(video_src: V, body_camera_rot3: Rotation3<f64>) -> Self {
        Self {
            body_camera_rot3,
            video_frame: None,
            video_src: video_src,
            state: Default::default(),
            zeniths: Default::default(),
        }
    }

    /// Builds a new [Solver] with unknown camera mount placement.
    /// Prefer [Self::new_fixed_body_camera] if you
    /// can determine the rotation matrix between the camera mount
    /// and the fixed rover body frame. The [Solver] will mitigate
    /// the mounting point coordinates by means of filter averaging.
    /// Expect errors in the first few solutions.
    /// 
    /// ## Inputs
    /// - video_src: [VideoSource] is a [Frame] provider.
    pub fn new(video_src: V) -> Self {
        Self {
            video_frame: None,
            video_src: video_src,
            state: Default::default(),
            zeniths: Default::default(),
            body_camera_rot3: Default::default(),
        }
    }

    /// Reset the navigation filter.
    /// Use this in case of a bumpy ride to restart the camera mounting
    /// point coordinates mitigation. Expect errors until the filter has converged.
    pub fn reset_nav_filter(&mut self) {
        self.body_camera_rot3 = Default::default();
    }

    /// Run [Solver] and try to resolve absolute 3D coordinates
    /// expressed in ECEF.
    ///
    /// Generics:
    ///   - K: constant kernel size (in pixels)
    ///   when isolating stars in [Frame] snapshot
    ///
    /// Inputs
    /// - mutable [Solver]
    /// - t: snapshot [Epoch]
    /// - orientation_rot3: rover current orientation, coming from IMU or navigation
    /// framework, expressed as [Rotation3]
    ///
    /// Outputs:
    /// - 3D coordinates update
    ///
    pub fn resolve<const K: usize>(&'a mut self, t: Epoch, orientation_rot3: Rotation3<f64>) -> Option<Coord> {
        match self.state {
            State::Capture => {
                self.video_capture();
                None
            }
            State::VideoProcessing => {
                self.video_processing::<K>(orientation_rot3);
                None
            }
            State::PostProcessing => self.post_processing(),
        }
    }

    /// Attempts to update the Video [Frame] snapshot.
    /// Call this any time the [VideoSource] is ready.
    /// This method is infaillible: if your [VideoSource] fails
    /// to provide a new [Frame] we'll just work on the previous one.
    pub fn video_capture(&'a mut self) {
        self.video_frame = self.video_src.next();
        self.state = State::VideoProcessing;
    }

    /// Run the star detection algorithm on latest video [Frame] snapshot
    pub fn video_processing<const K: usize>(&mut self, rot3: Rotation3<f64>) {
        // The algorithm is divded in several steps
        // - apply brightness threshold detector
        // - brightness histogram sorting and isolation
        // - finaly isolate central coordinates

        if self.video_frame.is_none() {
            self.state = State::Capture;
            return;
        }

        let mut frame = self.video_frame.as_ref().unwrap();

        // evaluate stars location within snapshot frame
        let coords = frame.star_coordinates_finder::<4>();
        
    }

    /// Run the 3D calculations and final projection
    ///
    /// ## Returns
    /// - [Coord] where x is the projected latitude (in radians)
    /// and y the projected longitude (in radians) of the rover.
    pub fn post_processing(&mut self) -> Option<Coord> {

        // form p matrix
        // [p0, p1, ... p_n] where p_i = cos(zenith_i);
        let p = Matrix1x4::from([
            self.zeniths[0].cos(),
            self.zeniths[1].cos(),
            self.zeniths[2].cos(),
            self.zeniths[3].cos(),
        ]);

        // // algebric solution (least square)
        // let a_t = a.transpose();

        // let a_a = (a_t * a).try_inverse();

        // if a_a.is_err() {
        //     // reset FSM (request new capture)
        //     self.state = State::Capture;
        //     return None;
        // }

        // let a_a = a_a.unwrap();

        // let x = a_a * a_t * p;

        // let lat_rad = (z / (x.powi(2) + y.powi(2)).sqrt()).atan();
        // let long_rad = (y / x).atan();

        // let coords = Coord::<f64>::from(&[lat_rad, long_rad]);

        self.state = State::Capture; // request new snapshot
        None
    }

}
