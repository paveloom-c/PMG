//! Rotation curve

use super::{Measurement, Object};
use crate::model::Consts;
use crate::utils::{compute_e_theta, compute_theta_r_g};

use std::fmt::{Debug, Display};

use anyhow::Result;
use num::Float;
use numeric_literals::replace_float_literals;

/// Azimuthal velocity (km/s)
#[derive(Debug)]
pub(in crate::model) struct AzimuthalVelocity<F: Float + Debug> {
    /// Measurement of the value (uncertainties here are
    /// inherited from the uncertainties of the parallax)
    pub(in crate::model) measurement: Measurement<F>,
    /// Uncertainty inherited from the velocities
    pub(in crate::model) e_vel: F,
}

/// Rotation curve
#[derive(Debug)]
pub(in crate::model) struct RotationCurve<F: Float + Debug> {
    /// Azimuthal velocity (km/s)
    pub(in crate::model) theta: AzimuthalVelocity<F>,
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
impl<F> RotationCurve<F>
where
    F: Float + Default + Display + Debug,
{
    pub(super) fn try_from(object: &Object<F>, consts: &Consts) -> Result<Self> {
        // Unpack the data
        let (alpha, delta) = object.equatorial_s()?.into();
        let (l, b) = object.galactic_s()?.into();
        let par = object.par()?;
        let v_lsr = object.v_lsr()?;
        let mu_x = object.mu_x()?;
        let mu_y = object.mu_y()?;
        // Compute the azimuthal velocity and the Galactocentric distance
        let (theta, r_g) =
            compute_theta_r_g(alpha, delta, l, b, par.v, v_lsr.v, mu_x.v, mu_y.v, consts);
        let (theta_u, r_g_u) =
            compute_theta_r_g(alpha, delta, l, b, par.v_u, v_lsr.v, mu_x.v, mu_y.v, consts);
        let (theta_l, r_g_l) =
            compute_theta_r_g(alpha, delta, l, b, par.v_l, v_lsr.v, mu_x.v, mu_y.v, consts);
        // Compute the uncertainty in the azimuthal velocity inherited from velocities
        let e_vel_theta = compute_e_theta(
            alpha, delta, l, b, par.v, v_lsr.v, mu_x.v, mu_y.v, v_lsr.e_p, mu_x.e_p, mu_y.e_p,
            consts,
        );
        Ok(Self {
            theta: AzimuthalVelocity {
                measurement: Measurement {
                    v: theta,
                    v_u: theta_u,
                    v_l: theta_l,
                    e_p: theta_u - theta,
                    e_m: theta - theta_l,
                },
                e_vel: e_vel_theta,
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

impl<'a, F: Float + Debug> From<&'a RotationCurve<F>>
    for (&'a AzimuthalVelocity<F>, &'a Measurement<F>)
{
    fn from(s: &'a RotationCurve<F>) -> Self {
        (&s.theta, &s.r_g)
    }
}
