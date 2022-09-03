//! Compute the uncertainty of the azimuthal velocity

use super::compute_theta_r_g::compute_theta;

use std::fmt::Debug;

use autodiff::FT;
use num::Float;
use numeric_literals::replace_float_literals;

/// Compute the uncertainty of the azimuthal velocity inherited from velocities
///
/// Sources: Gromov, Nikiforov, Ossipkov (2016)
#[allow(clippy::indexing_slicing)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::shadow_reuse)]
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::similar_names)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn compute_e_theta<F: Float + Debug>(
    alpha: F,
    delta: F,
    l: F,
    b: F,
    par: F,
    v_lsr: F,
    mu_x: F,
    mu_y: F,
    e_v_lsr: F,
    e_mu_x: F,
    e_mu_y: F,
) -> F {
    // Compute the partial derivative of the azimuthal
    // velocity by the Local Standard of Rest velocity
    let v: [FT<F>; 8] = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::cst(par),
        FT::var(v_lsr),
        FT::cst(mu_x),
        FT::cst(mu_y),
    ];
    let d_theta_v_lsr = compute_theta(&v).deriv();
    // Compute the partial derivative of the azimuthal
    // velocity by the Eastward proper motion
    let v: [FT<F>; 8] = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::cst(par),
        FT::cst(v_lsr),
        FT::var(mu_x),
        FT::cst(mu_y),
    ];
    let d_theta_mu_x = compute_theta(&v).deriv();
    // Compute the partial derivative of the azimuthal
    // velocity by the Northward proper motion
    let v: [FT<F>; 8] = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::cst(par),
        FT::cst(v_lsr),
        FT::cst(mu_x),
        FT::var(mu_y),
    ];
    let d_theta_mu_y = compute_theta(&v).deriv();
    // Compute the uncertainty
    F::sqrt(
        d_theta_v_lsr.powi(2) * e_v_lsr.powi(2)
            + d_theta_mu_x.powi(2) * e_mu_x.powi(2)
            + d_theta_mu_y.powi(2) * e_mu_y.powi(2),
    )
}
