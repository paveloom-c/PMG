//! Outer optimization problem

extern crate alloc;

use super::InnerOptimizationProblem;
use super::{Objects, Params};

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;

use anyhow::{Context, Result};
use argmin::core::{ArgminFloat, CostFunction, Executor, Gradient, State};
use argmin::solver::goldensectionsearch::GoldenSectionSearch;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use finitediff::FiniteDiff;
use num::Float;
use numeric_literals::replace_float_literals;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

/// A problem for the outer optimization
#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::type_complexity)]
pub(super) struct OuterOptimizationProblem<'a, F> {
    pub(super) objects: &'a Objects<F>,
    pub(super) params: &'a Params<F>,
    pub(super) par_pairs: &'a Rc<RefCell<Vec<(F, F, F)>>>,
}

/// Type of the parameters
pub type Param<F> = Vec<F>;

/// Type of the output
pub type Output<F> = F;

impl<'a, F> OuterOptimizationProblem<'a, F> {
    #[allow(clippy::as_conversions)]
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    /// Compute the parameterized part of the negative log likelihood function of the model
    pub(super) fn inner_cost(&self, p: &Param<F>, update_par_pairs: bool) -> Result<Output<F>>
    where
        F: Float
            + Debug
            + Default
            + Display
            + Sum
            + Sync
            + Send
            + ArgminFloat
            + ArgminL2Norm<F>
            + ArgminSub<F, F>
            + ArgminAdd<F, F>
            + ArgminDot<F, F>
            + ArgminMul<F, F>
            + ArgminZeroLike
            + ArgminMul<Vec<F>, Vec<F>>,
        Vec<F>: ArgminSub<Vec<F>, Vec<F>>,
        Vec<F>: ArgminSub<F, Vec<F>>,
        Vec<F>: ArgminAdd<Vec<F>, Vec<F>>,
        Vec<F>: ArgminAdd<F, Vec<F>>,
        Vec<F>: ArgminMul<F, Vec<F>>,
        Vec<F>: ArgminMul<Vec<F>, Vec<F>>,
        Vec<F>: ArgminL1Norm<F>,
        Vec<F>: ArgminSignum,
        Vec<F>: ArgminMinMax,
        Vec<F>: ArgminDot<Vec<F>, F>,
        Vec<F>: ArgminL2Norm<F>,
        Vec<F>: FiniteDiff,
    {
        // Unpack the problem
        let mut fit_objects = self.objects.clone();
        let mut fit_params = self.params.clone();
        // Update the parameters
        fit_params.update_with(p);
        // Prepare storage for the costs
        let mut costs = vec![F::zero(); self.objects.len()];
        // Compute the new value of the function
        fit_objects
            .par_iter_mut()
            .zip(costs.par_iter_mut())
            .zip(self.par_pairs.borrow_mut().par_iter_mut())
            .try_for_each(|((object, cost), par_pair)| -> Result<()> {
                // Compute some values
                object.compute_r_g(&fit_params);
                // Unpack the data
                let v_r = object.v_r.unwrap();
                let v_r_e = object.v_r_e.unwrap();
                let par = object.par.unwrap();
                let par_e = object.par_e.unwrap();
                let r_h = object.r_h.unwrap();
                let l = object.l.unwrap();
                let b = object.b.unwrap();
                let mu_l_cos_b = object.mu_l_cos_b.unwrap();
                let mu_b = object.mu_b.unwrap();
                let r_g = object.r_g.unwrap();
                // Unpack the parameters
                let r_0 = fit_params.r_0;
                let u_sun = fit_params.u_sun;
                let v_sun = fit_params.v_sun;
                let w_sun = fit_params.w_sun;
                let sigma_r_g = fit_params.sigma_r_g;
                let sigma_theta = fit_params.sigma_theta;
                let sigma_z = fit_params.sigma_z;
                let k = fit_params.k;
                // Compute the sines and cosines of the longitude and latitude
                let sin_l = l.sin();
                let sin_b = b.sin();
                let cos_l = l.cos();
                let cos_b = b.cos();
                // Compute their squares
                let sin_b_sq = sin_b.powi(2);
                let cos_l_sq = cos_l.powi(2);
                let cos_b_sq = cos_b.powi(2);
                // Compute the observed dispersions
                let d_r = sigma_r_g.powi(2);
                let d_theta = sigma_theta.powi(2);
                let d_z = sigma_z.powi(2);
                // Compute the sines and cosines of the Galactocentric longitude
                let sin_lambda = (r_h * cos_b) / r_g * sin_l;
                let cos_lambda = (r_0 - r_h * cos_b * cos_l) / r_g;
                // Compute the squares of the sines and cosines of the `phi` angle
                let sin_phi_sq = (sin_lambda * cos_l + cos_lambda * sin_l).powi(2);
                let cos_phi_sq = (cos_lambda * cos_l - sin_lambda * sin_l).powi(2);
                // Compute the natural dispersions
                let d_v_r_natural =
                    d_r * cos_phi_sq * cos_b_sq + d_theta * sin_phi_sq * cos_l_sq + d_z * sin_b_sq;
                let d_v_l_natural = d_r * sin_phi_sq + d_theta * cos_phi_sq;
                let d_v_b_natural =
                    d_r * cos_phi_sq * sin_b_sq + d_theta * sin_phi_sq * sin_b_sq + d_z * cos_b_sq;
                let delim = k.powi(2) * r_h.powi(2);
                let d_mu_l_cos_b_natural = d_v_l_natural / delim;
                let d_mu_b_natural = d_v_b_natural / delim;
                // Compute the dispersions of the observed proper motions
                let (d_mu_l_cos_b_observed, d_mu_b_observed) =
                    object.compute_d_mu_l_cos_b_mu_b(&fit_params);
                // Compute the full dispersions
                let d_v_r = v_r_e.powi(2) + d_v_r_natural;
                let d_mu_l_cos_b = d_mu_l_cos_b_observed + d_mu_l_cos_b_natural;
                let d_mu_b = d_mu_b_observed + d_mu_b_natural;
                let d_par = par_e.powi(2);
                // Compute the constant part of the model velocity
                let v_r_sun = -u_sun * cos_l * cos_b - v_sun * sin_l * cos_b - w_sun * sin_b;
                // Define a problem of the inner optimization
                let problem = InnerOptimizationProblem {
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
                    fit_params: &fit_params,
                };
                let init_param = par;
                let solver = GoldenSectionSearch::new(par - 5. * par_e, par + 5. * par_e)?
                    .with_tolerance(1e-10)?;
                // Find the local minimum in the inner optimization
                let res = Executor::new(problem, solver)
                    .configure(|state| state.param(init_param).max_iters(1000))
                    .run()
                    .with_context(|| "Couldn't solve the inner optimization problem")?;
                let &best_point = res.state().get_best_param().unwrap();
                let best_cost = res.state().get_best_cost();
                // Compute the final sum for this object
                *cost = F::ln(F::sqrt(d_v_r))
                    + F::ln(F::sqrt(d_mu_l_cos_b))
                    + F::ln(F::sqrt(d_mu_b))
                    + 0.5 * best_cost;
                // Save the results
                if update_par_pairs {
                    *par_pair = (par, par_e, best_point);
                }
                Ok(())
            })?;
        // We do the summing sequentially because
        // floating-point arithmetic is not associative
        let cost = costs.iter().copied().sum();
        // Return the value
        Ok(cost)
    }
}

impl<'a, F> CostFunction for OuterOptimizationProblem<'a, F>
where
    F: Float
        + Debug
        + Default
        + Display
        + Sync
        + Send
        + Sum
        + ArgminFloat
        + ArgminL2Norm<F>
        + ArgminSub<F, F>
        + ArgminAdd<F, F>
        + ArgminDot<F, F>
        + ArgminMul<F, F>
        + ArgminZeroLike
        + ArgminMul<Vec<F>, Vec<F>>,
    Vec<F>: ArgminSub<Vec<F>, Vec<F>>,
    Vec<F>: ArgminSub<F, Vec<F>>,
    Vec<F>: ArgminAdd<Vec<F>, Vec<F>>,
    Vec<F>: ArgminAdd<F, Vec<F>>,
    Vec<F>: ArgminMul<F, Vec<F>>,
    Vec<F>: ArgminMul<Vec<F>, Vec<F>>,
    Vec<F>: ArgminL1Norm<F>,
    Vec<F>: ArgminSignum,
    Vec<F>: ArgminMinMax,
    Vec<F>: ArgminDot<Vec<F>, F>,
    Vec<F>: ArgminL2Norm<F>,
    Vec<F>: FiniteDiff,
{
    type Param = Param<F>;
    type Output = Output<F>;

    #[allow(clippy::as_conversions)]
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn cost(&self, p: &Self::Param) -> Result<Self::Output> {
        self.inner_cost(p, true)
    }
}

impl<'a, F> Gradient for OuterOptimizationProblem<'a, F>
where
    F: Float
        + Debug
        + Default
        + Display
        + Sync
        + Send
        + Sum
        + ArgminFloat
        + ArgminL2Norm<F>
        + ArgminSub<F, F>
        + ArgminAdd<F, F>
        + ArgminDot<F, F>
        + ArgminMul<F, F>
        + ArgminZeroLike
        + ArgminMul<Vec<F>, Vec<F>>,
    Vec<F>: ArgminSub<Vec<F>, Vec<F>>,
    Vec<F>: ArgminSub<F, Vec<F>>,
    Vec<F>: ArgminAdd<Vec<F>, Vec<F>>,
    Vec<F>: ArgminAdd<F, Vec<F>>,
    Vec<F>: ArgminMul<F, Vec<F>>,
    Vec<F>: ArgminMul<Vec<F>, Vec<F>>,
    Vec<F>: ArgminL1Norm<F>,
    Vec<F>: ArgminSignum,
    Vec<F>: ArgminMinMax,
    Vec<F>: ArgminDot<Vec<F>, F>,
    Vec<F>: ArgminL2Norm<F>,
    Vec<F>: FiniteDiff,
{
    type Param = Vec<F>;
    type Gradient = Vec<F>;

    #[allow(clippy::unwrap_used)]
    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient> {
        Ok((*p).central_diff(&|x| self.inner_cost(x, false).unwrap().to_f64().unwrap()))
    }
}
