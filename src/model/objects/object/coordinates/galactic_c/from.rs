//! Convert the equatorial coordinates to
//! the Galactic coordinates and push them

use super::super::{EquatorialSpherical, GalacticCartesian};
use crate::utils::to_galactic;

use std::fmt::Debug;

use num::Float;

impl<F: Float + Debug> From<&EquatorialSpherical<F>> for GalacticCartesian<F> {
    fn from(equatorial: &EquatorialSpherical<F>) -> Self {
        // Convert to the Galactic heliocentric Cartesian coordinate system
        let (x, y, z) = to_galactic(equatorial.alpha, equatorial.delta, equatorial.par);
        GalacticCartesian { x, y, z }
    }
}
