//! Convert the equatorial spherical coordinates
//! to the Galactic spherical coordinates

use super::super::{EquatorialSpherical, GalacticSpherical};
use crate::utils::to_spherical;

use std::fmt::Debug;

use num::Float;

impl<F: Float + Debug> From<&EquatorialSpherical<F>> for GalacticSpherical<F> {
    fn from(equatorial_s: &EquatorialSpherical<F>) -> Self {
        // Unpack the data
        let alpha = equatorial_s.alpha;
        let delta = equatorial_s.delta;
        let par = equatorial_s.par;
        // Convert to the Galactic heliocentric spherical coordinate system
        let (l, b) = to_spherical(alpha, delta);
        Self { l, b, par }
    }
}
