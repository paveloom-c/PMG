//! Distances

use super::Object;
use crate::utils::compute_r_g;

use std::fmt::Debug;

use anyhow::Result;
use num::Float;
use numeric_literals::replace_float_literals;

/// Distances
#[derive(Debug)]
pub(in crate::model) struct Distances<F: Float> {
    /// Heliocentric distance (kpc)
    r_h: F,
    /// Galactocentric distance (kpc)
    ///
    /// Because of the different parameters being used,
    /// this is not the same distance as in the
    /// [`RotationCurve`](super::RotationCurve) struct
    r_g: F,
}

#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F: Float> TryFrom<&Object<F>> for Distances<F> {
    type Error = anyhow::Error;

    fn try_from(object: &Object<F>) -> Result<Self> {
        // Unpack the data
        let (l, b) = object.galactic_s()?.into();
        let par = object.par()?.v;
        // Compute the heliocentric distance
        let r_h = 1. / par;
        // Compute the Galactocentric distance
        let r_g = compute_r_g(l, b, r_h);
        Ok(Self { r_h, r_g })
    }
}

impl<F: Float> From<&Distances<F>> for (F, F) {
    fn from(s: &Distances<F>) -> Self {
        (s.r_h, s.r_g)
    }
}
