//! Compute the azimuthal velocity and Galactocentric distance

use crate::model::Params;

use core::fmt::Debug;

use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

/// Compute the azimuthal velocity and Galactocentric distance
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov, Ossipkov (2016)
#[allow(clippy::many_single_char_names)]
#[allow(clippy::shadow_reuse)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(<F as num::NumCast>::from(literal).unwrap())]
pub fn compute_theta<F, F2>(
    alpha: F,
    delta: F,
    l: F,
    b: F,
    r_h: F,
    r_g: F,
    v_lsr: F,
    mu_x: F,
    mu_y: F,
    params: &Params<F2>,
) -> F
where
    F: Float + Debug + FloatConst + From<F2>,
    F2: Float + Debug,
{
    // Get the parameters
    let u_sun_standard: F = params.u_sun_standard.into();
    let u_sun: F = params.u_sun.into();
    let theta_sun: F = params.theta_sun.into();
    let v_sun_standard: F = params.v_sun_standard.into();
    let w_sun_standard: F = params.w_sun_standard.into();
    let k: F = params.k.into();
    let r_0: F = params.r_0.into();
    // Compute the heliocentric velocity
    let v_r = v_lsr
        - (u_sun_standard * l.cos() + v_sun_standard * l.sin()) * b.cos()
        - w_sun_standard * b.sin();
    // Convert the proper motions in equatorial coordinates
    // to the proper motions in Galactic coordinates
    let (mu_l, mu_b) = super::compute_mu(alpha, delta, l, b, mu_x, mu_y, params);
    // Compute the linear velocities
    let v_l = k * r_h * mu_l * b.cos();
    let v_b = k * r_h * mu_b;
    // Convert the velocities to the Cartesian
    // heliocentric coordinate system
    let v_aux = v_r * b.cos() - v_b * b.sin();
    let u = v_aux * l.cos() - v_l * l.sin();
    let v = v_aux * l.sin() + v_l * l.cos();
    // Convert to the Galactocentric coordinate
    // system associated with the Sun
    let u_g = u + u_sun;
    let v_g = v + theta_sun;
    // Compute the projection of the heliocentric distance in the XY plane
    let d = r_h * b.cos();
    // Compute the azimuthal velocity
    let sin_lambda = d / r_g * l.sin();
    let cos_lambda = (r_0 - d * l.cos()) / r_g;
    v_g * cos_lambda + u_g * sin_lambda
}
