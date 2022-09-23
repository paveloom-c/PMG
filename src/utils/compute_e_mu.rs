//! Compute the dispersion of `mu_l * cos(b)`

use super::compute_mu_from;
use crate::model::{Measurement, Params};

use core::fmt::Debug;

use autodiff::FT;
use num::Float;

/// Compute the dispersion of `mu_l * cos(b)`
#[allow(clippy::shadow_unrelated)]
#[allow(clippy::similar_names)]
pub fn compute_e_mu<F: Float + Debug>(
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
