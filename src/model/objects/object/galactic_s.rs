//! Galactic heliocentric spherical coordinates

use super::Object;
use crate::utils::to_spherical;

use std::fmt::Debug;

use anyhow::Result;
use num::Float;
use numeric_literals::replace_float_literals;

/// Galactic heliocentric spherical coordinates
#[derive(Debug)]
pub(in crate::model) struct GalacticSpherical<F: Float + Debug> {
    /// Longitude (radians)
    pub(in crate::model) l: F,
    /// Latitude (radians)
    pub(in crate::model) b: F,
}

#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> TryFrom<&Object<F>> for GalacticSpherical<F>
where
    F: Float + Default + Debug,
{
    type Error = anyhow::Error;

    fn try_from(object: &Object<F>) -> Result<Self> {
        // Unpack the data
        let (alpha, delta) = object.equatorial_s()?.into();
        // Convert to the Galactic heliocentric spherical coordinate system
        let (l, b) = to_spherical(alpha, delta);
        Ok(Self { l, b })
    }
}

impl<F: Float + Debug> From<&GalacticSpherical<F>> for (F, F) {
    fn from(s: &GalacticSpherical<F>) -> Self {
        (s.l, s.b)
    }
}
