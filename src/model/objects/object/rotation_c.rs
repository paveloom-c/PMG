//! Rotation curve

use super::{Measurement, Object};
use crate::utils::{compute_e_theta, compute_r_g_2, compute_theta_r_g};

use std::fmt::Debug;

use anyhow::Result;
use num::Float;
use numeric_literals::replace_float_literals;

/// Rotation curve
#[derive(Debug)]
pub(in crate::model) struct RotationCurve<F: Float + Debug> {
    /// Azimuthal velocity (km/s)
    pub(in crate::model) theta: Measurement<F>,
    /// Galactocentric distance (kpc)
    ///
    /// Because of the different parameters being used,
    /// this is not the same distance as in the
    /// [`Distances`](super::Distances) struct
    pub(in crate::model) r_g: Measurement<F>,
}

#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F: Float + Debug> TryFrom<&Object<F>> for RotationCurve<F> {
    type Error = anyhow::Error;

    fn try_from(object: &Object<F>) -> Result<Self> {
        // Unpack the data
        let (alpha, delta) = object.equatorial_s()?.into();
        let (l, b) = object.galactic_s()?.into();
        let par = object.par()?;
        let v_lsr = object.v_lsr()?;
        let mu_x = object.mu_x()?;
        let mu_y = object.mu_y()?;
        // Compute the azimuthal velocity and the Galactocentric distance
        let (theta, r_g) = compute_theta_r_g(alpha, delta, l, b, par.v, v_lsr.v, mu_x.v, mu_y.v);
        // Compute the uncertainty in the azimuthal velocity
        let e_theta = compute_e_theta(
            alpha, delta, l, b, par.v, v_lsr.v, mu_x.v, mu_y.v, par.e_p, v_lsr.e_p, mu_x.e_p,
            mu_y.e_p,
        );
        // Compute the uncertainties in the Galactocentric distance
        let r_g_u = compute_r_g_2(l, b, 1. / par.v_u);
        let r_g_l = compute_r_g_2(l, b, 1. / par.v_l);
        Ok(Self {
            theta: Measurement {
                v: theta,
                v_u: theta + e_theta,
                v_l: theta - e_theta,
                e_p: e_theta,
                e_m: e_theta,
            },
            r_g: Measurement {
                v: r_g,
                v_u: r_g_u,
                v_l: r_g_l,
                e_p: r_g_u - r_g,
                e_m: r_g - r_g_l,
            },
        })
    }
}

impl<'a, F: Float + Debug> From<&'a RotationCurve<F>> for (&'a Measurement<F>, &'a Measurement<F>) {
    fn from(s: &'a RotationCurve<F>) -> Self {
        (&s.theta, &s.r_g)
    }
}
