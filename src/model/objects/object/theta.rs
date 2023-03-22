//! Rotation curve

use super::{Measurement, Object};
use crate::model::Params;
use crate::utils;

use core::fmt::{Debug, Display};

use anyhow::Result;
use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

/// Azimuthal velocity (km/s)
#[derive(Clone, Debug)]
pub(in crate::model) struct AzimuthalVelocity<F: Float + Debug> {
    /// Measurement of the value (uncertainties here are
    /// inherited from the uncertainties of the parallax)
    pub(in crate::model) m: Measurement<F>,
    /// Uncertainty inherited from the velocities
    pub(in crate::model) e_vel: F,
}

#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
impl<F> AzimuthalVelocity<F>
where
    F: Float + FloatConst + Default + Display + Debug,
{
    /// Try to convert the object into this struct
    pub(super) fn try_from(object: &Object<F>, params: &Params<F>) -> Result<Self> {
        // Unpack the data
        let (alpha, delta) = object.equatorial_s()?.into();
        let (r_h, l, b) = object.galactic_s()?.into();
        let r_g: &Measurement<F> = object.r_g()?.into();
        let par = object.par()?;
        let v_lsr = object.v_lsr()?;
        let mu_x = object.mu_x()?;
        let mu_y = object.mu_y()?;
        // Compute the azimuthal velocity
        let theta = utils::compute_theta(
            alpha, delta, l, b, r_h.v, r_g.v, v_lsr.v, mu_x.v, mu_y.v, params,
        );
        let theta_u = utils::compute_theta(
            alpha, delta, l, b, r_h.v_u, r_g.v_u, v_lsr.v, mu_x.v, mu_y.v, params,
        );
        let theta_l = utils::compute_theta(
            alpha, delta, l, b, r_h.v_l, r_g.v_l, v_lsr.v, mu_x.v, mu_y.v, params,
        );
        // Compute the uncertainty in the azimuthal velocity inherited from velocities
        let e_vel_theta =
            utils::compute_e_theta(alpha, delta, l, b, par.v, v_lsr, mu_x, mu_y, params);
        Ok(Self {
            m: Measurement {
                v: theta,
                v_u: theta_u,
                v_l: theta_l,
                e_p: theta_u - theta,
                e_m: theta - theta_l,
            },
            e_vel: e_vel_theta,
        })
    }
}
