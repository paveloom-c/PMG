//! Galactic heliocentric spherical coordinates

use super::Object;
use crate::model::Params;
use crate::utils;

use core::fmt::{Debug, Display};

use anyhow::Result;
use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

/// Galactic heliocentric spherical coordinates
#[derive(Clone, Debug)]
pub struct GalacticSpherical<F: Float + Debug> {
    /// Longitude (radians)
    pub l: F,
    /// Latitude (radians)
    pub b: F,
}

#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> GalacticSpherical<F>
where
    F: Float + FloatConst + Default + Display + Debug,
{
    /// Try to convert the object into this struct
    pub(super) fn try_from(object: &Object<F>, params: &Params<F>) -> Result<Self> {
        // Unpack the data
        let (alpha, delta) = object.equatorial_s()?.into();
        // Convert to the Galactic heliocentric spherical coordinate system
        let (l, b) = utils::to_spherical(alpha, delta, params);
        Ok(Self { l, b })
    }
}

impl<F: Float + Debug> From<&GalacticSpherical<F>> for (F, F) {
    fn from(s: &GalacticSpherical<F>) -> Self {
        (s.l, s.b)
    }
}
