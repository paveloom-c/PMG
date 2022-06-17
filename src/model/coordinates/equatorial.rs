//! Equatorial spherical coordinates

mod fun;

use std::fmt::Debug;

use num::Float;

/// Equatorial spherical coordinates
#[derive(Debug)]
pub(in crate::model) struct Equatorial<F: Float> {
    /// Right ascensions (radians)
    pub(super) alpha: Vec<F>,
    /// Declinations (radians)
    pub(super) delta: Vec<F>,
    /// Parallaxes (radians)
    pub(super) par: Vec<F>,
}
