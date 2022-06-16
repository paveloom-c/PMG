//! Galactic heliocentric coordinates

mod convert;

use crate::model::coordinates::Equatorial;
use convert::to_galactic;

use std::fmt::Debug;

use itertools::izip;
use num::Float;

/// Galactic heliocentric Cartesian coordinates
pub(in crate::model) struct Galactic<F: Float> {
    /// X coordinates
    pub(in crate::model) x: Vec<F>,
    /// Y coordinates
    pub(in crate::model) y: Vec<F>,
    /// Z coordinates
    pub(in crate::model) z: Vec<F>,
}

impl<F: Float> Galactic<F> {
    /// Create a new instance of the struct
    pub(in crate::model) fn new() -> Self {
        Self {
            x: Vec::<F>::new(),
            y: Vec::<F>::new(),
            z: Vec::<F>::new(),
        }
    }
    /// Extend the data of the struct
    pub(in crate::model) fn extend(&mut self, coords: Galactic<F>) {
        self.x.extend(coords.x);
        self.y.extend(coords.y);
        self.z.extend(coords.z);
    }
}

impl<F: Float + Debug> From<Equatorial<F>> for Galactic<F> {
    /// Convert the equatorial coordinates to
    /// the Galactic coordinates and push them
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::unwrap_used)]
    fn from(equatorial: Equatorial<F>) -> Self {
        izip!(equatorial.alpha, equatorial.delta, equatorial.par).fold(
            Galactic::new(),
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
