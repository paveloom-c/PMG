//! Inner optimization problem

use super::rotcurve::compute_rot_curve_series;
use super::{Object, Params};

use core::fmt::Debug;

use anyhow::Result;
use argmin::core::CostFunction;
use num::Float;
use numeric_literals::replace_float_literals;

/// A problem for the inner optimization
#[allow(clippy::missing_docs_in_private_items)]
pub(super) struct InnerOptimizationProblem<'a, F> {
    pub(super) l: F,
    pub(super) b: F,
    pub(super) v_sun: F,
    pub(super) v_r_sun: F,
    pub(super) v_r: F,
    pub(super) d_v_r: F,
    pub(super) mu_l_cos_b: F,
    pub(super) d_mu_l_cos_b: F,
    pub(super) mu_b: F,
    pub(super) d_mu_b: F,
    pub(super) par: F,
    pub(super) d_par: F,
    pub(super) fit_params: &'a Params<F>,
}

impl<'a, F> CostFunction for InnerOptimizationProblem<'a, F>
where
    F: Float + Debug + Default,
{
    type Param = F;
    type Output = F;

    // Find the reduced parallax
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn cost(&self, p: &Self::Param) -> Result<Self::Output> {
        let par_r = *p;
        // Unpack the problem
        let Self {
            l,
            b,
            v_sun,
            v_r_sun,
            v_r,
            d_v_r,
            mu_l_cos_b,
            d_mu_l_cos_b,
            mu_b,
            d_mu_b,
            par,
            d_par,
            fit_params,
        } = *self;
        // Create an object for the reduced values
        let mut object_r = Object {
            l: Some(l),
            b: Some(b),
            par: Some(par_r),
            ..Default::default()
        };
        // Compute the values
        object_r.compute_r_h_nominal();
        object_r.compute_r_g_nominal(fit_params);
        // Unpack the data
        let r_h_r = object_r.r_h.unwrap();
        let r_g_r = object_r.r_g.unwrap();
        // Unpack the parameters
        let Params {
            r_0,
            omega_0,
            u_sun,
            w_sun,
            k,
            ..
        } = *fit_params;
        // Compute the sines and cosines of the longitude and latitude
        let sin_l = l.sin();
        let sin_b = b.sin();
        let cos_l = l.cos();
        let cos_b = b.cos();
        // Compute the difference between the Galactocentric distances
        let delta_r_g = r_g_r - r_0;
        // Compute the sum of the terms in the series of the rotation curve
        let rot_curve_series = compute_rot_curve_series(delta_r_g, fit_params);
        // Compute the full model velocity
        let v_r_rot = rot_curve_series * r_0 / r_g_r * sin_l * cos_b;
        let v_r_mod = v_r_rot + v_r_sun;
        // Compute the model proper motion in longitude
        let mu_l_cos_b_rot =
            rot_curve_series * (r_0 * cos_l / r_h_r - cos_b) / r_g_r - omega_0 * cos_b;
        let mu_l_cos_b_sun = (u_sun * sin_l - v_sun * cos_l) / r_h_r;
        let mu_l_cos_b_mod = (mu_l_cos_b_rot + mu_l_cos_b_sun) / k;
        // Compute the model proper motion in latitude
        let mu_b_rot = -rot_curve_series * r_0 / r_g_r / r_h_r * sin_l * sin_b;
        let mu_b_sun = (u_sun * cos_l * sin_b + v_sun * sin_l * sin_b - w_sun * cos_b) / r_h_r;
        let mu_b_mod = (mu_b_rot + mu_b_sun) / k;
        // Compute the weighted sum of squared differences
        let sum = (v_r - v_r_mod).powi(2) / d_v_r
            + (mu_l_cos_b - mu_l_cos_b_mod).powi(2) / d_mu_l_cos_b
            + (mu_b - mu_b_mod).powi(2) / d_mu_b
            + (par - par_r).powi(2) / d_par;
        // Return it as the result
        Ok(sum)
    }
}
