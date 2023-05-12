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
use argmin::core::observers::{ObserverMode, SlogLogger};
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
pub struct ConfidenceIntervalProblem<'a, F> {
    pub n: usize,
    pub index: usize,
    pub best_outer_cost: F,
    pub objects: &'a Objects<F>,
    pub params: &'a Params<F>,
    pub triples: &'a Rc<RefCell<Vec<Triples<F>>>>,
    pub output_dir: &'a PathBuf,
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
    Vec<F>: FiniteDiff<F>,
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
            triples: self.triples,
            output_dir: self.output_dir,
        };
        let mut init_param = self.params.to_vec(self.n, false);
        // Remove the frozen parameter
        init_param.remove(self.index);
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
    ) -> Result<()> {
        // Make sure the fitting happened before this call
        let n = self.n.unwrap();
        let fit_params = self.fit_params.as_mut().unwrap();
        let best_point = fit_params.to_vec(n, false);
        // Prepare arrays for the confidence intervals
        let triple = vec![Triple::<F>::default(); 4];
        let triples = Rc::new(RefCell::new(vec![triple; self.objects.borrow().len()]));

        let len = 9 + (n - 1);
        let mut fit_params_ep = vec![F::zero(); len];
        let mut fit_params_em = vec![F::zero(); len];

        let tolerance = F::sqrt(F::epsilon());
        let max_iters = 100;

        // Define the confidence intervals
        izip!(&mut fit_params_ep, &mut fit_params_em)
            .enumerate()
            .try_for_each(|(index, (fit_param_ep, fit_param_em))| -> Result<()> {
                let param = best_point[index];

                writeln!(
                    errors_log_writer.borrow_mut(),
                    "index: {}, init_param: {param}",
                    index + 1,
                )?;

                eprintln!("index: {}", index + 1);

                // We compute the best value again since the
                // parameters are varied differently here
                let best_frozen_cost = {
                    let problem = FrozenOuterOptimizationProblem {
                        index,
                        param,
                        objects: &self.objects,
                        params: &self.params,
                        triples: &Rc::clone(&triples),
                        output_dir: &self.output_dir,
                    };
                    let mut init_param = self.params.to_vec(n, false);
                    // Remove the frozen parameter
                    init_param.remove(index);
                    let cond = ArmijoCondition::new(F::from(ARMIJO_PARAM).unwrap())?;
                    let linesearch = BacktrackingLineSearch::new(cond)
                        .rho(F::from(BACKTRACKING_PARAM).unwrap())?;
                    let solver = LBFGS::new(linesearch, LBFGS_M)
                        .with_tolerance_cost(F::from(LBFGS_TOLERANCE_ERRORS).unwrap())?;
                    // Find the local minimum in the outer optimization
                    let res = Executor::new(problem, solver)
                        .configure(|state| state.param(init_param))
                        .timer(false)
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
                'right: {
                    eprintln!("right");

                    writeln!(errors_log_writer.borrow_mut(), "\nto the right:")?;

                    let problem = ConfidenceIntervalProblem {
                        n,
                        index,
                        best_outer_cost: best_frozen_cost,
                        objects: &self.objects,
                        params: &self.params,
                        triples: &Rc::clone(&triples),
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
                        .add_observer(SlogLogger::term(), ObserverMode::Always)
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
                        break 'right;
                    }

                    let param_p = *res.unwrap().state().get_best_param().unwrap();
                    let diff_p = param_p - param;
                    *fit_param_ep = diff_p;

                    writeln!(errors_log_writer.borrow_mut(), "diff_p: {diff_p}")?;
                };

                // Find a root to the left
                'left: {
                    eprintln!("left");

                    writeln!(errors_log_writer.borrow_mut(), "\nto the left:")?;

                    let problem = ConfidenceIntervalProblem {
                        n,
                        index,
                        best_outer_cost: best_frozen_cost,
                        objects: &self.objects,
                        params: &self.params,
                        triples: &Rc::clone(&triples),
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
                        .add_observer(SlogLogger::term(), ObserverMode::Always)
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
                        break 'left;
                    }

                    let param_l = *res.unwrap().state().get_best_param().unwrap();
                    let diff_l = param - param_l;
                    *fit_param_em = diff_l;

                    writeln!(errors_log_writer.borrow_mut(), "diff_l: {diff_l}")?;
                };

                writeln!(errors_log_writer.borrow_mut())?;

                Ok(())
            })
            .with_context(|| "Couldn't define the confidence intervals")?;

        errors_log_writer.borrow_mut().flush()?;

        fit_params.update_ep_with(&fit_params_ep);
        fit_params.update_em_with(&fit_params_em);

        Ok(())
    }
}
