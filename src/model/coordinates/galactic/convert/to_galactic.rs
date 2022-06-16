//! Convert from equatorial coordinates to
//! Galactic heliocentric Cartesian coordinates

use super::{to_cartesian, to_spherical};
use std::fmt::Debug;

use num::Float;

/// Convert from equatorial coordinates to
/// Galactic heliocentric Cartesian coordinates
#[allow(clippy::many_single_char_names)]
pub(in super::super) fn to_galactic<F>(alpha: F, delta: F, par: F) -> (F, F, F)
where
    F: Float + Debug,
{
    // Convert to the spherical coordinate system
    let (l, b) = to_spherical(alpha, delta);
    // Convert to the Cartesian coordinate system
    let (x, y, z) = to_cartesian(l, b, par);
    (x, y, z)
}
