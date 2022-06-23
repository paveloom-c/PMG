//! Compute the uncertainty of the azimuthal velocity

use super::compute_theta_r_g;

use std::fmt::Debug;

use autodiff::FT;
use num::Float;

/// Compute the uncertainty of the azimuthal velocity
///
/// Sources: Gromov, Nikiforov, Ossipkov (2016)
#[allow(clippy::indexing_slicing)]
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::too_many_arguments)]
pub fn compute_e_theta<F: Float + Debug>(
    alpha: F,
    delta: F,
    l: F,
    b: F,
    par: F,
    v_lsr: F,
    mu_x: F,
    mu_y: F,
    e_par: F,
    e_v_lsr: F,
    e_mu_x: F,
    e_mu_y: F,
) -> F {
    // Define the object function
    let f = |v: &[FT<F>]| compute_theta_r_g(v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]).0;
    // Compute the partial derivative of the
    // azimuthal velocity by the parallax
    let v = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::var(par),
        FT::cst(v_lsr),
        FT::cst(mu_x),
        FT::cst(mu_y),
    ];
    let d_theta_par = f(&v).deriv();
    // Compute the partial derivative of the azimuthal
    // velocity by the Local Standard of Rest velocity
    let v = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::cst(par),
        FT::var(v_lsr),
        FT::cst(mu_x),
        FT::cst(mu_y),
    ];
    let d_theta_v_lsr = f(&v).deriv();
    // Compute the partial derivative of the azimuthal
    // velocity by the Eastward proper motion
    let v = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::cst(par),
        FT::cst(v_lsr),
        FT::var(mu_x),
        FT::cst(mu_y),
    ];
    let d_theta_mu_x = f(&v).deriv();
    // Compute the partial derivative of the azimuthal
    // velocity by the Northward proper motion
    let v = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::cst(par),
        FT::cst(v_lsr),
        FT::cst(mu_x),
        FT::var(mu_y),
    ];
    let d_theta_mu_y = f(&v).deriv();
    // Compute the uncertainty
    F::sqrt(
        d_theta_par.powi(2) * e_par.powi(2)
            + d_theta_v_lsr.powi(2) * e_v_lsr.powi(2)
            + d_theta_mu_x.powi(2) * e_mu_x.powi(2)
            + d_theta_mu_y.powi(2) * e_mu_y.powi(2),
    )
}
