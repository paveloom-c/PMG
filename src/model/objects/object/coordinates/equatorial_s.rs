//! Equatorial spherical coordinates

mod try_from;

use std::fmt::Debug;

use num::Float;

/// Equatorial spherical coordinates
#[derive(Debug)]
pub(in crate::model) struct EquatorialSpherical<F: Float> {
    /// Right ascension (radians)
    pub(in crate::model) alpha: F,
    /// Declination (radians)
    pub(in crate::model) delta: F,
    /// Parallax (mas)
    pub(in crate::model) par: F,
}
