//! Fit the model of the Galaxy to the data

extern crate alloc;

use super::Model;
use super::{
    ConfidenceIntervalProblem, FrozenOuterOptimizationProblem, Logger, OuterOptimizationProblem,
};

use alloc::rc::Rc;
use argmin::solver::linesearch::condition::ArmijoCondition;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;
use std::fs::File;

use anyhow::{Context, Result};
use argmin::core::observers::{ObserverMode, SlogLogger};
use argmin::core::{ArgminFloat, Executor, State};
use argmin::solver::brent::BrentRoot;
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
    /// Try to fit the model of the Galaxy to the data
    #[allow(clippy::as_conversions)]
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::non_ascii_literal)]
    #[allow(clippy::shadow_unrelated)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub(in crate::model) fn try_fit_from(&mut self, with_errors: bool) -> Result<()> {
        // Prepare a log file
        let log_path = self.output_dir.join("fit.log");
        File::create(log_path.clone()).with_context(|| "Couldn't create a log file")?;
        // Compute some of the values that don't
        // depend on the parameters being optimized
        self.objects.iter_mut().for_each(|object| {
            object.compute_l_b(&self.params);
            object.compute_v_r(&self.params);
            object.compute_r_h();
            object.compute_mu_l_cos_b_mu_b(&self.params);
        });
        // Define the problem of the outer optimization
        let par_pairs = Rc::new(RefCell::new(vec![(0., 0., 0.); self.objects.len()]));
        let problem = OuterOptimizationProblem {
            objects: &self.objects,
            params: &self.params,
            par_pairs: &Rc::clone(&par_pairs),
        };
        let init_param = self.params.to_point();
        let cond = ArmijoCondition::new(0.5)?;
        let linesearch = BacktrackingLineSearch::new(cond).rho(0.9)?;
        let solver = LBFGS::new(linesearch, 7).with_tolerance_cost(1e-12)?;
        // Find the local minimum in the outer optimization
        let res = Executor::new(problem, solver)
            .configure(|state| state.param(init_param))
            .add_observer(
                Logger {
                    params: self.params.clone(),
                    path: log_path,
                    par_pairs: Rc::clone(&par_pairs),
                },
                ObserverMode::Always,
            )
            .run()
            .with_context(|| "Couldn't solve the outer optimization problem")?;
        let best_point = res.state().get_best_param().unwrap().clone();
        // Prepare storage for the new parameters
        let mut fit_params = self.params.clone();
        fit_params.update_with(&best_point);
        // Compute the derived values
        self.params.theta_0 = self.params.r_0 * self.params.omega_0;
        self.params.theta_1 = self.params.omega_0 - 2. * self.params.a;
        self.params.theta_sun = self.params.theta_0 + self.params.v_sun;
        fit_params.theta_0 = fit_params.r_0 * fit_params.omega_0;
        fit_params.theta_1 = fit_params.omega_0 - 2. * fit_params.a;
        fit_params.theta_sun = fit_params.theta_0 + fit_params.v_sun;
        // Compute the uncertainties if requested
        if with_errors {
            // Prepare arrays for the confidence intervals
            let mut fit_params_ep = vec![0.; 9];
            let mut fit_params_em = vec![0.; 9];
            // Define the confidence intervals
            izip!(&mut fit_params_ep, &mut fit_params_em)
                .enumerate()
                .try_for_each(|(index, (fit_param_ep, fit_param_em))| -> Result<()> {
                    let param = best_point[index];

                    let tolerance = F::sqrt(F::epsilon());
                    let right_interval_widths = [2.0, 2.0, 2.0, 10.0, 1.5, 4.0, 2.0, 20.0, 2.0];
                    let left_interval_widths = [2.0, 2.0, 2.0, 10.0, 1.5, 4.0, 2.0, 20.0, 2.0];
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
                        let cond = ArmijoCondition::new(0.5)?;
                        let linesearch = BacktrackingLineSearch::new(cond).rho(0.9)?;
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

                    // Find a root to the right
                    {
                        let problem = ConfidenceIntervalProblem {
                            index,
                            best_outer_cost: best_frozen_cost,
                            objects: &self.objects,
                            params: &self.params,
                            par_pairs: &Rc::clone(&par_pairs),
                        };

                        let min = param;
                        let max = param + right_interval_widths[index];
                        let solver = BrentRoot::new(min, max, tolerance);

                        let res = Executor::new(problem, solver)
                            .configure(|state| state.param(param).max_iters(max_iters))
                            .add_observer(SlogLogger::term(), ObserverMode::Always)
                            .run()
                            .with_context(|| "Couldn't find a root to the right")?;

                        let param_p = *res.state().get_best_param().unwrap();
                        *fit_param_ep = param_p - param;
                    };

                    // Find a root to the left
                    {
                        let problem = ConfidenceIntervalProblem {
                            index,
                            best_outer_cost: best_frozen_cost,
                            objects: &self.objects,
                            params: &self.params,
                            par_pairs: &Rc::clone(&par_pairs),
                        };

                        let min = param - left_interval_widths[index];
                        let max = param;
                        let solver = BrentRoot::new(min, max, tolerance);

                        let res = Executor::new(problem, solver)
                            .configure(|state| state.param(param).max_iters(max_iters))
                            .add_observer(SlogLogger::term(), ObserverMode::Always)
                            .run()
                            .with_context(|| "Couldn't find a root to the left")?;

                        let param_l = *res.state().get_best_param().unwrap();
                        *fit_param_em = param - param_l;
                    };

                    Ok(())
                })
                .with_context(|| "Couldn't define the confidence intervals")?;
            fit_params.update_ep_with(&fit_params_ep);
            fit_params.update_em_with(&fit_params_em);
        }
        // Save the results
        self.fit_params = Some(fit_params);
        Ok(())
    }
}
