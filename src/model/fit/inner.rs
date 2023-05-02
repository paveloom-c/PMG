//! Inner optimization problem

use super::rotcurve::compute_rot_curve_series;
use super::{Object, Params};

use core::fmt::Debug;

use anyhow::Result;
use argmin::core::CostFunction;
use num::Float;

/// A triple of the discrepancy
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Default, Clone)]
pub struct Triple<F> {
    pub observed: F,
    pub model: F,
    pub error: F,
}

/// Triples of the discrepancies
pub type Triples<F> = Vec<Triple<F>>;

/// A problem for the inner optimization
#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct InnerOptimizationProblem<'a, F> {
    pub l: F,
    pub b: F,
    pub v_sun: F,
    pub v_r_sun: F,
    pub v_r: F,
    pub v_r_error: F,
    pub mu_l_cos_b: F,
    pub mu_l_cos_b_error: F,
    pub mu_b: F,
    pub mu_b_error: F,
    pub par: F,
    pub par_e: F,
    pub fit_params: &'a Params<F>,
}

impl<'a, F> InnerOptimizationProblem<'a, F> {
    /// Compute the discrepancies
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::unwrap_used)]
    pub fn compute_triples(&self, par_r: F) -> Triples<F>
    where
        F: Float + Debug + Default,
    {
        // Unpack the problem
        let Self {
            l,
            b,
            v_sun,
            v_r_sun,
            v_r,
            v_r_error,
            mu_l_cos_b,
            mu_l_cos_b_error,
            mu_b,
            mu_b_error,
            par,
            par_e,
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
        // Return the triples
        vec![
            Triple {
                observed: v_r,
                model: v_r_mod,
                error: v_r_error,
            },
            Triple {
                observed: mu_l_cos_b,
                model: mu_l_cos_b_mod,
                error: mu_l_cos_b_error,
            },
            Triple {
                observed: mu_b,
                model: mu_b_mod,
                error: mu_b_error,
            },
            Triple {
                observed: par,
                model: par_r,
                error: par_e,
            },
        ]
    }
}

impl<'a, F> CostFunction for InnerOptimizationProblem<'a, F>
where
    F: Float + Debug + Default,
{
    type Param = F;
    type Output = F;

    // Find the reduced parallax
    fn cost(&self, p: &Self::Param) -> Result<Self::Output> {
        let par_r = *p;
        // Compute the discrepancies
        let triples = self.compute_triples(par_r);
        // Compute the sum
        let mut sum = F::zero();
        for triple in triples {
            // We don't use the function below here because there is a slight difference in the
            // squared values, which sometimes leads to huge difference in the results
            sum = sum + (triple.observed - triple.model).powi(2) / triple.error.powi(2);
        }
        Ok(sum)
    }
}

/// Compute the relative discrepancy from a triplet
pub fn compute_relative_discrepancy<F>(triple: &Triple<F>) -> F
where
    F: Float + Debug,
{
    (triple.observed - triple.model).abs() / triple.error
}
