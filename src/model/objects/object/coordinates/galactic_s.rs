//! Galactic heliocentric spherical coordinates

mod from;

use std::fmt::Debug;

use num::Float;

/// Galactic heliocentric spherical coordinates
#[derive(Debug)]
pub(in crate::model) struct GalacticSpherical<F: Float> {
    /// Longitude (radians)
    pub(in crate::model) l: F,
    /// Latitude (radians)
    pub(in crate::model) b: F,
    /// Parallax (mas)
    pub(in crate::model) par: F,
    /// Heliocentric distance (kpc)
    pub(in crate::model) r_h: F,
    /// Galactocentric distance (kpc)
    pub(in crate::model) r_g: F,
}
