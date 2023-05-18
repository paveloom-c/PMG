//! Outer optimization problem

extern crate alloc;

use super::{prepare_inner_problem, InnerOptimizationProblem, Triples};
use super::{Objects, Params};
use crate::utils::{self, FiniteDiff};

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;
use itertools::izip;
use std::path::PathBuf;

use anyhow::{Context, Result};
use argmin::core::{ArgminFloat, CostFunction, Executor, Gradient, State};
use argmin::solver::brent::BrentOpt;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use num::Float;
use numeric_literals::replace_float_literals;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

/// A problem for the outer optimization
#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::type_complexity)]
pub struct OuterOptimizationProblem<'a, F> {
    pub disable_inner: bool,
    pub objects: &'a Objects<F>,
    pub params: &'a Params<F>,
    pub triples: &'a Rc<RefCell<Vec<Triples<F>>>>,
    pub output_dir: &'a PathBuf,
}

/// Type of the parameters
pub type Param<F> = Vec<F>;

/// Type of the output
pub type Output<F> = F;

impl<'a, F> OuterOptimizationProblem<'a, F> {
    #[allow(clippy::as_conversions)]
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::pattern_type_mismatch)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    /// Compute the parameterized part of the negative log likelihood function of the model
    pub fn inner_cost(&self, p: &Param<F>, update_triples: bool) -> Result<Output<F>>
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
        Vec<F>: FiniteDiff<F>,
    {
        // Unpack the problem
        let mut fit_params = self.params.clone();
        let disable_inner = self.disable_inner;
        // Update the parameters
        fit_params.update_with(p);
        // Prepare storage for the costs
        let mut costs = vec![F::zero(); self.objects.borrow().len()];
        // Compute the new value of the function
        self.objects
            .borrow_mut()
            .par_iter_mut()
            .zip(costs.par_iter_mut())
            .zip(self.triples.borrow_mut().par_iter_mut())
            .try_for_each(|((object, cost), triple)| -> Result<()> {
                // Skip if the object is an outlier
                if object.outlier {
                    return Ok(());
                };

                // Compute some values
                object.compute_r_g(&fit_params);
                // Unpack the data
                let par = object.par.unwrap();
                let par_e = object.par_e.unwrap();
                // Define a problem of the inner optimization
                let problem = prepare_inner_problem(object, &fit_params);

                let (best_par_r, best_sum) = if disable_inner {
                    let sum = problem.cost(&par)?;
                    (par, sum)
                } else {
                    // Scan the vicinity of the observed parallax via subintervals

                    let mut pars = Vec::with_capacity(5);
                    let mut sums = Vec::with_capacity(5);

                    let n_subintervals = 50;
                    for coeff in [1., 2., 3.] {
                        find_minima(
                            &problem,
                            &mut pars,
                            &mut sums,
                            par + (coeff - 1.) * 3. * par_e,
                            par + coeff * 3. * par_e,
                            n_subintervals,
                        )?;
                        find_minima(
                            &problem,
                            &mut pars,
                            &mut sums,
                            F::max(F::epsilon(), par - coeff * 3. * par_e),
                            par - (coeff - 1.) * 3. * par_e,
                            n_subintervals,
                        )?;
                        if !pars.is_empty() {
                            break;
                        }
                    }

                    // Find the result closest to the observed parallax
                    //
                    // The default values are like this because of
                    // the initialization stage of the executor
                    let mut best_par_r = 0.;
                    let mut best_sum = if pars.is_empty() { 0. } else { F::infinity() };
                    for (par_r, sum) in izip!(&pars, &sums) {
                        if *sum < best_sum {
                            best_par_r = *par_r;
                            best_sum = *sum;
                        }
                    }

                    (best_par_r, best_sum)
                };

                *cost = F::ln(problem.v_r_error)
                    + F::ln(problem.mu_l_cos_b_error)
                    + F::ln(problem.mu_b_error)
                    + 0.5 * best_sum;

                if update_triples {
                    *triple = problem.compute_triples(best_par_r);
                }

                Ok(())
            })?;
        // We do the summing sequentially because
        // floating-point arithmetic is not associative
        let cost = costs.iter().copied().sum();
        Ok(cost)
    }
}

/// Find minima in the interval
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn find_minima<F>(
    problem: &InnerOptimizationProblem<'_, F>,
    pars: &mut Vec<F>,
    sums: &mut Vec<F>,
    start: F,
    end: F,
    n_subintervals: usize,
) -> Result<()>
where
    F: Float
        + Debug
        + Default
        + Display
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
    Vec<F>: FiniteDiff<F>,
{
    let h_subintervals = (end - start) / F::from(n_subintervals).unwrap();

    let diff_epsilon = F::sqrt(F::epsilon());
    let mut subinterval_start_diff =
        utils::central_diff(start, &|x| problem.cost(&x).unwrap(), diff_epsilon);

    for j in 0..=n_subintervals {
        let subinterval_start = start + F::from(j).unwrap() * h_subintervals;
        let subinterval_end = subinterval_start + h_subintervals;

        let subinterval_end_diff = utils::central_diff(
            subinterval_end,
            &|x| problem.cost(&x).unwrap(),
            diff_epsilon,
        );

        if subinterval_start_diff < 0. && subinterval_end_diff > 0. {
            let init_param = (subinterval_end - subinterval_start) / 2.;
            let solver = BrentOpt::new(subinterval_start, subinterval_end)
                .set_tolerance(F::sqrt(F::epsilon()), 1e-15);
            let res = Executor::new(problem.clone(), solver)
                .configure(|state| state.param(init_param).max_iters(100))
                .timer(false)
                .run()
                .with_context(|| "Couldn't solve the inner optimization problem")?;

            let &par_r = res.state().get_best_param().unwrap();
            let sum = res.state().get_best_cost();

            pars.push(par_r);
            sums.push(sum);
        }

        subinterval_start_diff = subinterval_end_diff;
    }

    Ok(())
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
    Vec<F>: FiniteDiff<F>,
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
    Vec<F>: FiniteDiff<F>,
{
    type Param = Vec<F>;
    type Gradient = Vec<F>;

    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient> {
        Ok((*p).central_diff(
            &|x| self.inner_cost(x, false).unwrap(),
            F::sqrt(F::epsilon()),
        ))
    }
}
