//! Compute the dispersion of `mu_l * cos(b)`

use crate::model::{Measurement, Params};

use core::fmt::Debug;

use autodiff::FT;
use num::{traits::FloatConst, Float};

/// Compute proper motions in equatorial
/// coordinates from the array of arguments
pub fn compute_mu_from<F: Float + FloatConst + Debug + Default>(
    args: &[FT<F>; 6],
    params: &Params<F>,
) -> (FT<F>, FT<F>) {
    // Alias the arguments
    let alpha = args[0];
    let delta = args[1];
    let l = args[2];
    let b = args[3];
    let mu_x = args[4];
    let mu_y = args[5];
    // Compute proper motions in equatorial coordinates
    super::compute_mu(alpha, delta, l, b, mu_x, mu_y, params)
}

/// Compute the dispersions of `mu_l * cos(b)` and `mu_b`
///
/// Note that only values with independent errors are in the parameters.
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::similar_names)]
pub fn compute_e_mu<F: Float + FloatConst + Debug + Default>(
    alpha: F,
    delta: F,
    l: F,
    b: F,
    mu_x: &Measurement<F>,
    mu_y: &Measurement<F>,
    params: &Params<F>,
) -> (F, F) {
    // Compute the observed dispersions
    let d_mu_x = mu_x.e_p.powi(2);
    let d_mu_y = mu_y.e_p.powi(2);
    // Compute the partial derivatives of
    // `mu_l * cos(b)` by `mu_alpha * cos(delta)`
    // and `mu_b` by `mu_alpha * cos(delta)`
    let v: [FT<F>; 6] = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::var(mu_x.v),
        FT::cst(mu_y.v),
    ];
    let mu = compute_mu_from(&v, params);
    let deriv_mu_l_cos_b_mu_x_sq = mu.0.deriv().powi(2);
    let deriv_mu_b_mu_x_sq = mu.1.deriv().powi(2);
    // Compute the partial derivatives of
    // `mu_l * cos(b)` by `mu_delta`
    // and `mu_b` by `mu_delta`
    let v: [FT<F>; 6] = [
        FT::cst(alpha),
        FT::cst(delta),
        FT::cst(l),
        FT::cst(b),
        FT::cst(mu_x.v),
        FT::var(mu_y.v),
    ];
    let mu = compute_mu_from(&v, params);
    let deriv_mu_l_cos_b_mu_y_sq = mu.0.deriv().powi(2);
    let deriv_mu_b_mu_y_sq = mu.1.deriv().powi(2);
    // Compute the dispersion of `mu_l * cos(b)`
    let sigma_mu_l_cos_b_sq = deriv_mu_l_cos_b_mu_x_sq * d_mu_x + deriv_mu_l_cos_b_mu_y_sq * d_mu_y;
    // Compute the dispersion of `mu_b`
    let sigma_mu_b_sq = deriv_mu_b_mu_x_sq * d_mu_x + deriv_mu_b_mu_y_sq * d_mu_y;
    // Return the results
    (sigma_mu_l_cos_b_sq, sigma_mu_b_sq)
}
