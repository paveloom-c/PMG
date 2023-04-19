//! Fit the model of the Galaxy to the data

extern crate alloc;

use super::Model;
use super::{FitLogger, OuterOptimizationProblem};

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
use finitediff::FiniteDiff;
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
    pub(in crate::model) fn try_fit_params(&mut self) -> Result<()> {
        // Prepare the fit log file
        let logs_dir_path = self.output_dir.join("logs");
        let fit_log_path = logs_dir_path.join("fit.log");
        std::fs::create_dir_all(logs_dir_path)
            .with_context(|| "Couldn't create the logs directory")?;
        let fit_log_file =
            File::create(fit_log_path).with_context(|| "Couldn't create the `fit.log` file")?;
        let fit_log_writer = Rc::new(RefCell::new(BufWriter::new(fit_log_file)));
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
        let cond = ArmijoCondition::new(1e-4)?;
        let linesearch = BacktrackingLineSearch::new(cond).rho(0.5)?;
        let solver = LBFGS::new(linesearch, 7).with_tolerance_cost(1e-12)?;
        // Find the local minimum in the outer optimization
        let res = Executor::new(problem, solver)
            .configure(|state| state.param(init_param))
            .add_observer(
                FitLogger {
                    params: self.params.clone(),
                    par_pairs: Rc::clone(&par_pairs),
                    writer: fit_log_writer,
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
        // Save the results
        self.fit_params = Some(fit_params);
        Ok(())
    }
}
