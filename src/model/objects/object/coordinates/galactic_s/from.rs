//! Convert the equatorial spherical coordinates
//! to the Galactic spherical coordinates

use super::super::{EquatorialSpherical, GalacticSpherical};
use crate::utils::compute_r_g;
use crate::utils::to_spherical;

use num::Float;
use numeric_literals::replace_float_literals;

#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F: Float> From<&EquatorialSpherical<F>> for GalacticSpherical<F> {
    fn from(equatorial_s: &EquatorialSpherical<F>) -> Self {
        // Unpack the data
        let alpha = equatorial_s.alpha;
        let delta = equatorial_s.delta;
        let par = equatorial_s.par;
        // Convert to the Galactic heliocentric spherical coordinate system
        let (l, b) = to_spherical(alpha, delta);
        // Compute the heliocentric distance
        let r_h = 1. / par;
        // Compute the Galactocentric distance
        let r_g = compute_r_g(l, b, r_h);
        Self {
            l,
            b,
            par,
            r_h,
            r_g,
        }
    }
}
