//! Galactic heliocentric Cartesian coordinates

mod from;

use std::fmt::Debug;

use num::Float;

/// Galactic heliocentric Cartesian coordinates
#[derive(Debug)]
pub(in crate::model) struct GalacticCartesian<F: Float> {
    /// X coordinate
    pub(in crate::model) x: F,
    /// Y coordinate
    pub(in crate::model) y: F,
    /// Z coordinate
    pub(in crate::model) z: F,
}
