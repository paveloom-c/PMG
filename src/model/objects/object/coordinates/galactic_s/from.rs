//! Convert the equatorial spherical coordinates
//! to the Galactic spherical coordinates

use super::super::{EquatorialSpherical, GalacticSpherical};
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
        // Compute the distance
        let r = 1. / par;
        // Convert to the Galactic heliocentric spherical coordinate system
        let (l, b) = to_spherical(alpha, delta);
        Self { l, b, par, r }
    }
}
