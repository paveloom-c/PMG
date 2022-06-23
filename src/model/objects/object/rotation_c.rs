//! Rotation curve

use super::Object;
use crate::utils::compute_theta_r;

use std::fmt::Debug;

use anyhow::Result;
use num::Float;

/// Rotation curve
#[derive(Debug)]
pub(in crate::model) struct RotationCurve<F: Float + Debug> {
    /// Azimuthal velocity (km/s)
    pub(in crate::model) theta: F,
    /// Galactocentric distance (kpc)
    ///
    /// Because of the different parameters being used,
    /// this is not the same distance as in the
    /// [`Distances`](super::Distances) struct
    pub(in crate::model) r: F,
}

impl<F: Float + Debug> TryFrom<&Object<F>> for RotationCurve<F> {
    type Error = anyhow::Error;

    fn try_from(object: &Object<F>) -> Result<Self> {
        // Unpack the data
        let equatorial_s = object.equatorial_s()?;
        let (l, b) = object.galactic_s()?.into();
        let alpha = equatorial_s.alpha;
        let delta = equatorial_s.delta;
        // Compute the azimuthal velocity and Galactocentric distance
        let (theta, r) = compute_theta_r(alpha, delta, l, b);
        Ok(Self { theta, r })
    }
}
