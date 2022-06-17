//! Convert the equatorial coordinates to
//! the Galactic coordinates and push them

use crate::model::coordinates::{Equatorial, Galactic};
use crate::utils::to_galactic;

use std::fmt::Debug;

use itertools::izip;
use num::Float;

impl<F: Float + Debug> From<Equatorial<F>> for Galactic<F> {
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::unwrap_used)]
    fn from(equatorial: Equatorial<F>) -> Self {
        izip!(equatorial.alpha, equatorial.delta, equatorial.par).fold(
            Galactic::default(),
            |mut acc, (alpha, delta, par)| {
                // Convert to the Galactic heliocentric Cartesian coordinate system
                let (x, y, z) = to_galactic(alpha, delta, par);
                acc.x.push(x);
                acc.y.push(y);
                acc.z.push(z);
                acc
            },
        )
    }
}
