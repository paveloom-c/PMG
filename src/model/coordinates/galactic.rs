//! Galactic heliocentric Cartesian coordinates

mod fun;

use std::fmt::Debug;

use num::Float;

/// Galactic heliocentric Cartesian coordinates
#[derive(Debug)]
pub(in crate::model) struct Galactic<F: Float> {
    /// X coordinates
    pub(in crate::model) x: Vec<F>,
    /// Y coordinates
    pub(in crate::model) y: Vec<F>,
    /// Z coordinates
    pub(in crate::model) z: Vec<F>,
}
