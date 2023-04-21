//! Confidence intervals (standard errors)

extern crate alloc;

use super::{ErrorsLogger, FrozenOuterOptimizationProblem};
use super::{Model, Objects, Params};

use alloc::rc::Rc;
use argmin::core::observers::ObserverMode;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::{Context, Result};
use argmin::core::{ArgminFloat, CostFunction, Executor, State};
use argmin::solver::brent::BrentRoot;
use argmin::solver::linesearch::condition::ArmijoCondition;
use argmin::solver::linesearch::BacktrackingLineSearch;
use argmin::solver::quasinewton::LBFGS;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use finitediff::FiniteDiff;
use itertools::izip;
use num::Float;
use numeric_literals::replace_float_literals;

#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::type_complexity)]
pub(super) struct ConfidenceIntervalProblem<'a, F> {
    pub(super) index: usize,
    pub(super) best_outer_cost: F,
    pub(super) objects: &'a Objects<F>,
    pub(super) params: &'a Params<F>,
    pub(super) par_pairs: &'a Rc<RefCell<Vec<(F, F, F)>>>,
}

impl<'a, F> CostFunction for ConfidenceIntervalProblem<'a, F>
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
    type Param = F;
    type Output = F;

    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn cost(&self, param: &Self::Param) -> Result<Self::Output> {
        // Define the problem of the outer optimization with a frozen parameter
        let problem = FrozenOuterOptimizationProblem {
            index: self.index,
            param: *param,
            objects: self.objects,
            params: self.params,
            par_pairs: self.par_pairs,
        };
        let mut init_param = self.params.to_point();
        // Remove the frozen parameter
        init_param.remove(self.index);
        let cond = ArmijoCondition::new(1e-4)?;
        let linesearch = BacktrackingLineSearch::new(cond).rho(0.5)?;
        let solver = LBFGS::new(linesearch, 7).with_tolerance_cost(1e-12)?;
        // Find the local minimum in the outer optimization
        let res = Executor::new(problem, solver)
            .configure(|state| state.param(init_param))
            .run()
            .with_context(|| {
                "Couldn't solve the outer optimization problem with a frozen parameter"
            })?;
        let best_inner_cost = res.state().get_best_cost();
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
    Vec<F>: FiniteDiff,
{
    /// Try to define the confidence intervals
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub(in crate::model) fn try_fit_errors(&mut self) -> Result<()> {
        // Make sure the fitting happened before this call
        let fit_params = self.fit_params.as_mut().unwrap();
        let best_point = fit_params.to_point();
        // Prepare the errors log file
        let errors_log_path = self.output_dir.join("errors.log");
        let errors_log_file = File::create(errors_log_path)
            .with_context(|| "Couldn't create the `errors.log` file")?;
        let errors_log_writer = Rc::new(RefCell::new(BufWriter::new(errors_log_file)));
        // Prepare arrays for the confidence intervals
        let par_pairs = Rc::new(RefCell::new(vec![(0., 0., 0.); self.objects.len()]));
        let mut fit_params_ep = vec![0.; 9];
        let mut fit_params_em = vec![0.; 9];
        // Define the confidence intervals
        izip!(&mut fit_params_ep, &mut fit_params_em)
            .enumerate()
            .try_for_each(|(index, (fit_param_ep, fit_param_em))| -> Result<()> {
                let param = best_point[index];

                writeln!(
                    errors_log_writer.borrow_mut(),
                    "index: {index}, init_param: {param}"
                )?;

                let tolerance = F::sqrt(F::epsilon());
                let right_interval_widths = [2.0, 2.0, 2.0, 10.0, 2.0, 4.0, 5.0, 20.0, 2.0];
                let left_interval_widths = [2.0, 2.0, 2.0, 10.0, 2.0, 4.0, 5.0, 20.0, 2.0];
                let max_iters = 100;

                // We compute the best value again since the
                // parameters are varied differently here
                let best_frozen_cost = {
                    let problem = FrozenOuterOptimizationProblem {
                        index,
                        param,
                        objects: &self.objects,
                        params: &self.params,
                        par_pairs: &Rc::clone(&par_pairs),
                    };
                    let mut init_param = self.params.to_point();
                    // Remove the frozen parameter
                    init_param.remove(index);
                    let cond = ArmijoCondition::new(1e-4)?;
                    let linesearch = BacktrackingLineSearch::new(cond).rho(0.5)?;
                    let solver = LBFGS::new(linesearch, 7).with_tolerance_cost(1e-12)?;
                    // Find the local minimum in the outer optimization
                    let res = Executor::new(problem, solver)
                        .configure(|state| state.param(init_param))
                        .run()
                        .with_context(|| {
                            "Couldn't solve the outer optimization problem with a frozen parameter"
                        })?;
                    res.state().get_best_cost()
                };

                writeln!(
                    errors_log_writer.borrow_mut(),
                    "best_frozen_cost: {best_frozen_cost}"
                )?;

                // Find a root to the right
                {
                    writeln!(errors_log_writer.borrow_mut(), "\nto the right:")?;

                    let problem = ConfidenceIntervalProblem {
                        index,
                        best_outer_cost: best_frozen_cost,
                        objects: &self.objects,
                        params: &self.params,
                        par_pairs: &Rc::clone(&par_pairs),
                    };

                    let min = param;
                    let max = param + right_interval_widths[index];
                    let cost_min = problem.cost(&min)?;
                    let cost_max = problem.cost(&max)?;

                    writeln!(
                        errors_log_writer.borrow_mut(),
                        "min: {min}, max: {max}, cost_min: {cost_min}, cost_max: {cost_max}"
                    )?;

                    let solver = BrentRoot::new(min, max, tolerance);

                    let res = Executor::new(problem, solver)
                        .configure(|state| state.param(param).max_iters(max_iters))
                        .add_observer(
                            ErrorsLogger {
                                writer: Rc::clone(&errors_log_writer),
                            },
                            ObserverMode::Always,
                        )
                        .run()
                        .with_context(|| "Couldn't find a root to the right")?;

                    let param_p = *res.state().get_best_param().unwrap();
                    let diff_p = param_p - param;
                    *fit_param_ep = diff_p;

                    writeln!(errors_log_writer.borrow_mut(), "diff_p: {diff_p}")?;
                };

                // Find a root to the left
                {
                    writeln!(errors_log_writer.borrow_mut(), "\nto the left:")?;

                    let problem = ConfidenceIntervalProblem {
                        index,
                        best_outer_cost: best_frozen_cost,
                        objects: &self.objects,
                        params: &self.params,
                        par_pairs: &Rc::clone(&par_pairs),
                    };

                    let min = param - left_interval_widths[index];
                    let max = param;
                    let cost_min = problem.cost(&min)?;
                    let cost_max = problem.cost(&max)?;

                    writeln!(
                        errors_log_writer.borrow_mut(),
                        "min: {min}, max: {max}, cost_min: {cost_min}, cost_max: {cost_max}"
                    )?;

                    let solver = BrentRoot::new(min, max, tolerance);

                    let res = Executor::new(problem, solver)
                        .configure(|state| state.param(param).max_iters(max_iters))
                        .add_observer(
                            ErrorsLogger {
                                writer: Rc::clone(&errors_log_writer),
                            },
                            ObserverMode::Always,
                        )
                        .run()
                        .with_context(|| "Couldn't find a root to the left")?;

                    let param_l = *res.state().get_best_param().unwrap();
                    let diff_l = param - param_l;
                    *fit_param_em = diff_l;

                    writeln!(errors_log_writer.borrow_mut(), "diff_l: {diff_l}")?;
                };

                writeln!(errors_log_writer.borrow_mut())?;

                Ok(())
            })
            .with_context(|| "Couldn't define the confidence intervals")?;
        // Save the results
        fit_params.update_ep_with(&fit_params_ep);
        fit_params.update_em_with(&fit_params_em);
        Ok(())
    }
}
