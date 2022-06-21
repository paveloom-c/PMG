//! Equatorial spherical coordinates

mod try_from;

use std::fmt::Debug;

use num::Float;

/// Equatorial spherical coordinates
#[derive(Debug)]
pub(in crate::model) struct EquatorialSpherical<F: Float> {
    /// Right ascension (radians)
    pub(super) alpha: F,
    /// Declination (radians)
    pub(super) delta: F,
    /// Parallax (mas)
    pub(super) par: F,
}
