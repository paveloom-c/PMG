//! Rotation curve

use super::Object;
use crate::utils::compute_theta_r_g;

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
    pub(in crate::model) r_g: F,
}

impl<F: Float + Debug> TryFrom<&Object<F>> for RotationCurve<F> {
    type Error = anyhow::Error;

    fn try_from(object: &Object<F>) -> Result<Self> {
        // Unpack the data
        let (alpha, delta) = object.equatorial_s()?.into();
        let (l, b) = object.galactic_s()?.into();
        let (r_h, _) = object.distances()?.into();
        let v = object.v()?;
        let mu_x = object.mu_x()?;
        let mu_y = object.mu_y()?;
        // Compute the azimuthal velocity and Galactocentric distance
        let (theta, r_g) = compute_theta_r_g(alpha, delta, l, b, r_h.v, v.v, mu_x.v, mu_y.v);
        Ok(Self { theta, r_g })
    }
}

impl<F: Float + Debug> From<&RotationCurve<F>> for (F, F) {
    fn from(s: &RotationCurve<F>) -> Self {
        (s.theta, s.r_g)
    }
}
