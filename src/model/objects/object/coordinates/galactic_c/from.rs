//! Convert the Galactic spherical coordinates
//! to the Galactic Cartesian coordinates

use super::super::{GalacticCartesian, GalacticSpherical};
use crate::utils::to_cartesian;

use num::Float;

#[allow(clippy::many_single_char_names)]
impl<F: Float> From<&GalacticSpherical<F>> for GalacticCartesian<F> {
    fn from(galactic_s: &GalacticSpherical<F>) -> Self {
        // Unpack the data
        let l = galactic_s.l;
        let b = galactic_s.b;
        let par = galactic_s.par;
        // Convert to the Galactic heliocentric Cartesian coordinate system
        let (x, y, z) = to_cartesian(l, b, par);
        Self { x, y, z }
    }
}
