//! Fit the model of the Galaxy to the data

extern crate alloc;

use super::Model;
use super::{FitLogger, OuterOptimizationProblem, SigmaOuterOptimizationProblem};
use crate::utils::FiniteDiff;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;
use std::fs::File;
use std::io::BufWriter;

use anyhow::{Context, Result};
use argmin::core::observers::ObserverMode;
use argmin::core::{ArgminFloat, Executor, State};
use argmin::solver::linesearch::condition::ArmijoCondition;
use argmin::solver::linesearch::BacktrackingLineSearch;
use argmin::solver::quasinewton::LBFGS;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use num::Float;
use numeric_literals::replace_float_literals;

/// A parameter for the Armijo condition
pub const ARMIJO_PARAM: f64 = 1e-4;

/// A parameter for the backtracking search
pub const BACKTRACKING_PARAM: f64 = 0.9;

/// Memory length of the L-BFGS algorithm
pub const LBFGS_M: usize = 300;

/// Tolerance of the L-BFGS algorithm for the errors
pub const LBFGS_TOLERANCE_ERRORS: f64 = 1e-11;

/// Maximum number of iterations
pub const MAX_ITERS: u64 = 500;

/// A term to add to the velocities of non-from-Reid objects
pub const VEL_TERM: f64 = 10.;

impl<F> Model<F> {
    /// Try to fit the model of the Galaxy to the data
    #[allow(clippy::as_conversions)]
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::non_ascii_literal)]
    #[allow(clippy::print_stderr)]
    #[allow(clippy::shadow_unrelated)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn try_fit_params(
        &mut self,
        n: usize,
        sample_iteration: usize,
        l_stroke: usize,
        fit_log_writer: &Rc<RefCell<BufWriter<File>>>,
    ) -> Result<()>
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
        // Compute some of the values that don't
        // depend on the parameters being optimized
        self.objects.borrow_mut().iter_mut().for_each(|object| {
            object.compute_l_b(&self.params);
            object.compute_v_r(&self.params);
            object.compute_r_h();
            object.compute_mu_l_cos_b_mu_b(&self.params);
        });

        let (best_cost, mut fit_params) = if l_stroke == 3 {
            // Define the problem of the outer optimization
            let problem = OuterOptimizationProblem {
                disable_inner: self.disable_inner,
                objects: &self.objects,
                params: &self.params,
                triples: &Rc::clone(&self.triples),
                output_dir: &self.output_dir,
            };
            // Find the local minimum in the outer optimization
            let init_param = self.params.to_vec(n, false);
            let cond = ArmijoCondition::new(F::from(ARMIJO_PARAM).unwrap())?;
            let linesearch =
                BacktrackingLineSearch::new(cond).rho(F::from(BACKTRACKING_PARAM).unwrap())?;
            let solver = LBFGS::new(linesearch, LBFGS_M)
                .with_tolerance_cost(F::from(self.lbfgs_tolerance).unwrap())?;
            let res = Executor::new(problem, solver)
                .configure(|state| state.param(init_param).max_iters(MAX_ITERS))
                .timer(false)
                .add_observer(
                    FitLogger {
                        l_stroke,
                        sample_iteration,
                        objects: Rc::clone(&self.objects),
                        params: self.params.clone(),
                        triples: Rc::clone(&self.triples),
                        writer: Rc::clone(fit_log_writer),
                    },
                    ObserverMode::Always,
                )
                .run()
                .with_context(|| "Couldn't solve the outer optimization problem")?;

            let best_cost = res.state().get_best_cost();
            let best_point = res.state().get_best_param().unwrap().clone();

            // Prepare storage for the new parameters
            let mut fit_params = self.params.clone();
            fit_params.update_with(&best_point);

            (best_cost, fit_params)
        } else {
            let mut fit_params = self.fit_params.as_ref().unwrap().clone();
            // Define the problem of the outer optimization
            let problem = SigmaOuterOptimizationProblem {
                disable_inner: self.disable_inner,
                objects: &self.objects,
                fit_params: &fit_params,
                triples: &Rc::clone(&self.triples),
                output_dir: &self.output_dir,
            };
            // Find the local minimum in the outer optimization
            let init_param = self.params.to_vec(n, true);
            let cond = ArmijoCondition::new(F::from(ARMIJO_PARAM).unwrap())?;
            let linesearch =
                BacktrackingLineSearch::new(cond).rho(F::from(BACKTRACKING_PARAM).unwrap())?;
            let solver = LBFGS::new(linesearch, LBFGS_M)
                .with_tolerance_cost(F::from(self.lbfgs_tolerance).unwrap())?;
            let res = Executor::new(problem, solver)
                .configure(|state| state.param(init_param).max_iters(MAX_ITERS))
                .timer(false)
                .add_observer(
                    FitLogger {
                        l_stroke,
                        sample_iteration,
                        objects: Rc::clone(&self.objects),
                        params: self.params.clone(),
                        triples: Rc::clone(&self.triples),
                        writer: Rc::clone(fit_log_writer),
                    },
                    ObserverMode::Always,
                )
                .run()
                .with_context(|| "Couldn't solve the outer optimization problem")?;

            let best_cost = res.state().get_best_cost();
            let mut best_point = res.state().get_best_param().unwrap().clone();

            best_point.insert(6, fit_params.sigma_r_g);
            best_point.insert(7, fit_params.sigma_theta);
            best_point.insert(8, fit_params.sigma_z);

            // Update the previous parameters
            fit_params.update_with(&best_point);

            (best_cost, fit_params)
        };
        // Compute the derived values
        self.params.theta_0 = self.params.r_0 * self.params.omega_0;
        self.params.theta_1 = self.params.omega_0 - 2. * self.params.a;
        self.params.theta_sun = self.params.theta_0 + self.params.v_sun;
        self.params.omega_sun = self.params.omega_0 + self.params.v_sun / self.params.r_0;
        fit_params.theta_0 = fit_params.r_0 * fit_params.omega_0;
        fit_params.theta_1 = fit_params.omega_0 - 2. * fit_params.a;
        fit_params.theta_sun = fit_params.theta_0 + fit_params.v_sun;
        fit_params.omega_sun = fit_params.omega_0 + fit_params.v_sun / fit_params.r_0;
        // Save the results
        self.n = Some(n);
        self.best_cost = Some(best_cost);
        self.fit_params = Some(fit_params);
        Ok(())
    }
}
