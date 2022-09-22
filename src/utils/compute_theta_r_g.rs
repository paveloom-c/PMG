//! Compute the azimuthal velocity and Galactocentric distance

use super::{compute_mu, compute_r_g};
use crate::model::Params;

use std::fmt::Debug;

use autodiff::FT;
use num::Float;
use numeric_literals::replace_float_literals;

/// Compute the azimuthal velocity from the array of arguments
pub fn compute_theta<F: Float + Debug>(args: &[FT<F>; 8], params: &Params<F>) -> FT<F> {
    compute_theta_r_g(
        args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], params,
    )
    .0
}

/// Compute the azimuthal velocity and Galactocentric distance
///
/// Sources: Reid et al. (2009); Gromov, Nikiforov, Ossipkov (2016)
#[allow(clippy::many_single_char_names)]
#[allow(clippy::shadow_reuse)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(<F as num::NumCast>::from(literal).unwrap())]
pub fn compute_theta_r_g<F, F2>(
    alpha: F,
    delta: F,
    l: F,
    b: F,
    par: F,
    v_lsr: F,
    mu_x: F,
    mu_y: F,
    params: &Params<F2>,
) -> (F, F)
where
    F: Float + Debug + From<F2>,
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
    let v_h = v_lsr
        - (u_sun_standard * l.cos() + v_sun_standard * l.sin()) * b.cos()
        - w_sun_standard * b.sin();
    // Convert the proper motions in equatorial coordinates
    // to the proper motions in Galactic coordinates
    let (mu_l, mu_b) = compute_mu(alpha, delta, l, b, mu_x, mu_y, params);
    // Compute the heliocentric distance
    let r_h = 1. / par;
    // Compute the linear velocities
    let v_l = k * r_h * mu_l * b.cos();
    let v_b = k * r_h * mu_b;
    // Convert the velocities to the Cartesian
    // heliocentric coordinate system
    let v_aux = v_h * b.cos() - v_b * b.sin();
    let u = v_aux * l.cos() - v_l * l.sin();
    let v = v_aux * l.sin() + v_l * l.cos();
    // Convert to the Galactocentric coordinate
    // system associated with the Sun
    let u_g = u + u_sun;
    let v_g = v + theta_sun;
    // Compute the projection of the heliocentric distance in the XY plane
    let d = r_h * b.cos();
    // Compute the Galactocentric distance
    let r_g = compute_r_g(l, b, r_h, params);
    // Compute the azimuthal velocity
    let sin_lambda = d / r_g * l.sin();
    let cos_lambda = (r_0 - d * l.cos()) / r_g;
    let theta = v_g * cos_lambda + u_g * sin_lambda;
    (theta, r_g)
}
