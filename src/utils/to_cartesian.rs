//! Convert heliocentric Galactic coordinates from
//! the spherical to the Cartesian coordinate system

use num::Float;
use numeric_literals::replace_float_literals;

/// Convert the heliocentric Galactic coordinates
/// from the spherical to the Cartesian coordinate system
///
/// Angles must be in radians.
#[allow(clippy::many_single_char_names)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub(super) fn to_cartesian<F: Float>(l: F, b: F, par: F) -> (F, F, F) {
    // Compute the distance in `kpc`
    let d = 1. / par;
    // Convert to the Galactic heliocentric Cartesian system
    let x = d * F::cos(b) * F::cos(l);
    let y = d * F::cos(b) * F::sin(l);
    let z = d * F::sin(b);
    (x, y, z)
}
