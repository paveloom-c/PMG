//! Computation goal

use core::cmp::Ord;

/// Computation goal
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Goal {
    /// Convert equatorial coordinates to the Galactic
    /// heliocentric spherical and Cartesian coordinates
    Coords,
    /// Compute the rotation curve
    RotationCurve,
    /// Fit the model of the Galaxy to the data
    Fit,
}
