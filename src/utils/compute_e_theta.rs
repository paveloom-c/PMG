//! Compute the uncertainty of the azimuthal velocity

use crate::model::{Measurement, Params};

use core::fmt::Debug;

use autodiff::FT;
use num::{traits::FloatConst, Float};
use numeric_literals::replace_float_literals;

/// Compute the azimuthal velocity from the array of arguments
#[allow(clippy::unwrap_used)]
#[replace_float_literals(<FT::<F> as num::NumCast>::from(literal).unwrap())]
pub fn compute_theta_from<F: Float + FloatConst + Debug>(
    args: &[FT<F>; 8],
    params: &Params<F>,
) -> FT<F> {
    // Alias the arguments
    let alpha = args[0];
    let delta = args[1];
    let l = args[2];
    let b = args[3];
    let par = args[4];
    let v_lsr = args[5];
    let mu_x = args[6];
    let mu_y = args[7];
    // Compute the heliocentric distance
    let r_h = 1. / par;
    // Compute the Galactocentric distance
    let r_g = super::compute_r_g(l, b, r_h, params);
    // Compute the azimuthal velocity
    super::compute_theta(alpha, delta, l, b, r_h, r_g, v_lsr, mu_x, mu_y, params)
}

/// Compute the uncertainty of the azimuthal velocity inherited from velocities
///
/// Note that only values with independent errors are in the parameters.
///
/// Sources: Gromov, Nikiforov, Ossipkov (2016)
#[allow(clippy::indexing_slicing)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::shadow_reuse)]
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::similar_names)]
#[allow(clippy::too_many_arguments)]
pub fn compute_e_theta<F: Float + FloatConst + Debug>(
    alpha: F,
    delta: F,
    l: F,
    b: F,
    par: F,
    v_lsr: &Measurement<F>,
    mu_x: &Measurement<F>,
    mu_y: &Measurement<F>,
    params: &Params<F>,
) -> F {
    // Compute the partial derivative of the azimuthal
    // velocity by the Local Standard of Rest velocity
    let v: [FT<F>; 8] = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::cst(par),
        FT::var(v_lsr.v),
        FT::cst(mu_x.v),
        FT::cst(mu_y.v),
    ];
    let d_theta_v_lsr = compute_theta_from(&v, params).deriv();
    // Compute the partial derivative of the azimuthal
    // velocity by the Eastward proper motion
    let v: [FT<F>; 8] = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::cst(par),
        FT::cst(v_lsr.v),
        FT::var(mu_x.v),
        FT::cst(mu_y.v),
    ];
    let d_theta_mu_x = compute_theta_from(&v, params).deriv();
    // Compute the partial derivative of the azimuthal
    // velocity by the Northward proper motion
    let v: [FT<F>; 8] = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::cst(par),
        FT::cst(v_lsr.v),
        FT::cst(mu_x.v),
        FT::var(mu_y.v),
    ];
    let d_theta_mu_y = compute_theta_from(&v, params).deriv();
    // Compute the uncertainty
    F::sqrt(
        d_theta_v_lsr.powi(2) * v_lsr.e_p.powi(2)
            + d_theta_mu_x.powi(2) * mu_x.e_p.powi(2)
            + d_theta_mu_y.powi(2) * mu_y.e_p.powi(2),
    )
}
