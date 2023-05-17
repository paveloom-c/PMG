//! Confidence intervals (standard errors)

extern crate alloc;

use super::params::{ARMIJO_PARAM, BACKTRACKING_PARAM, LBFGS_M, LBFGS_TOLERANCE_ERRORS, MAX_ITERS};
use super::{ErrorsLogger, FrozenOuterOptimizationProblem, Triple, Triples};
use super::{Model, Objects, Params};
use crate::utils::FiniteDiff;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use argmin::core::observers::ObserverMode;
use argmin::core::{ArgminFloat, CostFunction, Executor, State};
use argmin::solver::brent::BrentRoot;
use argmin::solver::linesearch::condition::ArmijoCondition;
use argmin::solver::linesearch::BacktrackingLineSearch;
use argmin::solver::quasinewton::LBFGS;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use itertools::izip;
use num::Float;
use numeric_literals::replace_float_literals;

#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::type_complexity)]
pub struct ConfidenceIntervalProblem<'a, F, FN>
where
    FN: Fn(F, &[F]) -> F,
{
    pub l_stroke: usize,
    pub n: usize,
    pub index: usize,
    pub best_outer_cost: F,
    pub objects: &'a Objects<F>,
    pub params: &'a Params<F>,
    pub compute_param: FN,
    pub fit_params: &'a Params<F>,
    pub triples: &'a Rc<RefCell<Vec<Triples<F>>>>,
    pub output_dir: &'a PathBuf,
}

impl<'a, F, FN> ConfidenceIntervalProblem<'a, F, FN>
where
    FN: Fn(F, &[F]) -> F,
{
    /// Optimize the outer problem with one (or four) parameters frozen
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn inner_cost(&self, param: &F) -> Result<F>
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
        let index = if self.l_stroke == 1 {
            if self.index < 6 {
                self.index
            } else {
                self.index - 3
            }
        } else {
            self.index
        };

        // Remove the frozen parameter
        let mut init_param = self.params.to_vec(self.n, self.l_stroke == 1);
        init_param.remove(index);
        // Define the problem of the outer optimization with a frozen parameter
        let problem = FrozenOuterOptimizationProblem {
            l_stroke: self.l_stroke,
            index,
            param: *param,
            compute_param: &self.compute_param,
            objects: self.objects,
            params: self.params,
            fit_params: self.fit_params,
            triples: self.triples,
            output_dir: self.output_dir,
        };
        let cond = ArmijoCondition::new(F::from(ARMIJO_PARAM).unwrap())?;
        let linesearch =
            BacktrackingLineSearch::new(cond).rho(F::from(BACKTRACKING_PARAM).unwrap())?;
        let solver = LBFGS::new(linesearch, LBFGS_M)
            .with_tolerance_cost(F::from(LBFGS_TOLERANCE_ERRORS).unwrap())?;
        // Find the local minimum in the outer optimization
        let res = Executor::new(problem, solver)
            .configure(|state| state.param(init_param).max_iters(MAX_ITERS))
            .timer(false)
            .run()
            .with_context(|| {
                "Couldn't solve the outer optimization problem with a frozen parameter"
            })?;
        Ok(res.state().get_best_cost())
    }
}

impl<'a, F, FN> CostFunction for ConfidenceIntervalProblem<'a, F, FN>
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
    FN: Fn(F, &[F]) -> F,
{
    type Param = F;
    type Output = F;

    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn cost(&self, param: &Self::Param) -> Result<Self::Output> {
        let best_inner_cost = self.inner_cost(param)?;
        Ok(best_inner_cost - self.best_outer_cost - 0.5)
    }
}

impl<F> Model<F>
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
    /// Try to define the confidence intervals
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::print_stderr)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::use_debug)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn try_fit_errors(
        &mut self,
        errors_log_writer: &Rc<RefCell<BufWriter<File>>>,
        l_stroke: usize,
    ) -> Result<()> {
        let n = self.n.unwrap();

        let triple = vec![Triple::<F>::default(); 4];
        let triples = Rc::new(RefCell::new(vec![triple; self.objects.borrow().len()]));

        let best_point = self.fit_params.as_ref().unwrap().to_vec(n, false);
        let mut fit_params_ep = self.fit_params.as_ref().unwrap().to_ep_vec(n);
        let mut fit_params_em = self.fit_params.as_ref().unwrap().to_em_vec(n);

        // Define the confidence intervals
        izip!(&mut fit_params_ep, &mut fit_params_em)
            .enumerate()
            .try_for_each(|(index, (fit_param_ep, fit_param_em))| -> Result<()> {
                // Don't compute for the sigmas or compute for the sigmas only
                if Params::<F>::compute_with_l_stroke(index, l_stroke) {
                    return Ok(());
                }

                let param = best_point[index];

                // Don't do anything specific with the `param`
                let compute_param = |x: F, _: &[F]| x;

                let (diff_p, diff_m) = self.try_fit_errors_pair(
                    l_stroke,
                    index,
                    param,
                    compute_param,
                    errors_log_writer,
                    &triples,
                )?;

                *fit_param_ep = diff_p;
                *fit_param_em = diff_m;

                Ok(())
            })
            .with_context(|| "Couldn't define the confidence intervals")?;

        errors_log_writer.borrow_mut().flush()?;

        {
            let fit_params = self.fit_params.as_mut().unwrap();
            fit_params.update_ep_with(&fit_params_ep);
            fit_params.update_em_with(&fit_params_em);
        }

        // Compute errors for `omega_sun`, too, by
        // temporarily changing the parametrization
        if l_stroke == 1 {
            writeln!(
                errors_log_writer.borrow_mut(),
                "errors for `omega_sun` while mimicking under `omega_0`",
            )?;

            let param = self.fit_params.as_ref().unwrap().omega_sun;

            // Compute `omega_0` from `omega_sun` = `v_sun` / `R_0`
            //
            // The index is 3 and not 4 because the
            // frozen parameter (index 1) is removed
            let compute_param = |omega_sun: F, p: &[F]| omega_sun - p[3] / p[0];

            let (diff_p, diff_m) = self.try_fit_errors_pair(
                l_stroke,
                1,
                param,
                compute_param,
                errors_log_writer,
                &triples,
            )?;

            let fit_params = self.fit_params.as_mut().unwrap();
            fit_params.omega_sun_ep = diff_p;
            fit_params.omega_sun_em = diff_m;
        }

        Ok(())
    }
    /// Try to define a confidence interval of one parameter
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::print_stderr)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::use_debug)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn try_fit_errors_pair<FN>(
        &self,
        l_stroke: usize,
        index: usize,
        param: F,
        compute_param: FN,
        errors_log_writer: &Rc<RefCell<BufWriter<File>>>,
        triples: &Rc<RefCell<Vec<Triples<F>>>>,
    ) -> Result<(F, F)>
    where
        FN: Fn(F, &[F]) -> F,
    {
        let tolerance = F::sqrt(F::epsilon());
        let max_iters = 100;

        let n = self.n.unwrap();
        let fit_params = self.fit_params.as_ref().unwrap();

        writeln!(
            errors_log_writer.borrow_mut(),
            "index: {}, init_param: {param}",
            index + 1,
        )?;

        // We compute the best value again since the
        // parameters are varied differently here
        let best_frozen_cost = {
            let problem = ConfidenceIntervalProblem {
                l_stroke,
                n,
                index,
                best_outer_cost: F::zero(),
                objects: &self.objects,
                params: &self.params,
                compute_param: &compute_param,
                fit_params,
                triples: &Rc::clone(triples),
                output_dir: &self.output_dir,
            };
            problem.inner_cost(&param)?
        };

        writeln!(
            errors_log_writer.borrow_mut(),
            "best_frozen_cost: {best_frozen_cost}"
        )?;

        // Find a root to the right
        let diff_p: F = 'right: {
            writeln!(errors_log_writer.borrow_mut(), "\nto the right:")?;

            let problem = ConfidenceIntervalProblem {
                l_stroke,
                n,
                index,
                best_outer_cost: best_frozen_cost,
                objects: &self.objects,
                params: &self.params,
                compute_param: &compute_param,
                fit_params,
                triples: &Rc::clone(triples),
                output_dir: &self.output_dir,
            };

            let min = param;
            let mut max = param + 3.;
            let cost_min = problem.cost(&min)?;
            let mut cost_max = problem.cost(&max)?;

            if cost_min * cost_max > 0. {
                max = max + 3.;
                cost_max = problem.cost(&max)?;
            }

            writeln!(
                errors_log_writer.borrow_mut(),
                "min: {min}, max: {max}, cost_min: {cost_min}, cost_max: {cost_max}"
            )?;

            let solver = BrentRoot::new(min, max, tolerance);

            let res = Executor::new(problem, solver)
                .configure(|state| state.param(param).max_iters(max_iters))
                .timer(false)
                .add_observer(
                    ErrorsLogger {
                        writer: Rc::clone(errors_log_writer),
                    },
                    ObserverMode::Always,
                )
                .run()
                .with_context(|| "Couldn't find a root to the right");
            if let Err(ref err) = res {
                eprintln!("{err:?}");
                break 'right F::zero();
            }

            let param_p = *res.unwrap().state().get_best_param().unwrap();
            let diff_p = param_p - param;
            writeln!(errors_log_writer.borrow_mut(), "diff_p: {diff_p}")?;

            diff_p
        };

        // Find a root to the left
        let diff_l: F = 'left: {
            writeln!(errors_log_writer.borrow_mut(), "\nto the left:")?;

            let problem = ConfidenceIntervalProblem {
                l_stroke,
                n,
                index,
                best_outer_cost: best_frozen_cost,
                objects: &self.objects,
                params: &self.params,
                compute_param,
                fit_params,
                triples: &Rc::clone(triples),
                output_dir: &self.output_dir,
            };

            let mut min = param - 3.;
            let max = param;
            let mut cost_min = problem.cost(&min)?;
            let cost_max = problem.cost(&max)?;

            if cost_min * cost_max > 0. {
                min = min - 3.;
                cost_min = problem.cost(&min)?;
            }

            writeln!(
                errors_log_writer.borrow_mut(),
                "min: {min}, max: {max}, cost_min: {cost_min}, cost_max: {cost_max}"
            )?;

            let solver = BrentRoot::new(min, max, tolerance);

            let res = Executor::new(problem, solver)
                .configure(|state| state.param(param).max_iters(max_iters))
                .timer(false)
                .add_observer(
                    ErrorsLogger {
                        writer: Rc::clone(errors_log_writer),
                    },
                    ObserverMode::Always,
                )
                .run()
                .with_context(|| "Couldn't find a root to the left");
            if let Err(ref err) = res {
                eprintln!("{err:?}");
                break 'left F::zero();
            }

            let param_l = *res.unwrap().state().get_best_param().unwrap();
            let diff_l = param - param_l;

            writeln!(errors_log_writer.borrow_mut(), "diff_l: {diff_l}")?;

            diff_l
        };

        writeln!(errors_log_writer.borrow_mut())?;

        Ok((diff_p, diff_l))
    }
}
