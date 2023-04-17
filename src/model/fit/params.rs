//! Fit the model of the Galaxy to the data

extern crate alloc;

use super::Model;
use super::{Logger, OuterOptimizationProblem};

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;
use std::fs::File;

use anyhow::{Context, Result};
use argmin::core::observers::ObserverMode;
use argmin::core::{ArgminFloat, Executor, State};
use argmin::solver::linesearch::MoreThuenteLineSearch;
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
    pub(in crate::model) fn try_fit_from(&mut self) -> Result<()> {
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
            par_pairs: Rc::clone(&par_pairs),
        };
        let init_param = self.params.to_point();
        let linesearch = MoreThuenteLineSearch::new();
        let solver = LBFGS::new(linesearch, 7);
        // Find the local minimum in the outer optimization
        let res = Executor::new(problem, solver)
            .configure(|state| state.param(init_param))
            // .add_observer(SlogLogger::term(), ObserverMode::Always)
            .add_observer(
                Logger {
                    params: self.params.clone(),
                    path: log_path,
                    par_pairs,
                },
                ObserverMode::Always,
            )
            .run()?;
        let best_point = res.state().get_best_param().unwrap();
        // let best_cost = res.state().get_best_cost();
        // Prepare storage for the new parameters
        let mut fit_params = self.params.clone();
        fit_params.update_with(best_point);
        // Compute the derived values
        self.params.theta_0 = self.params.r_0 * self.params.omega_0;
        self.params.theta_sun = self.params.theta_0 + self.params.v_sun;
        fit_params.theta_0 = fit_params.r_0 * fit_params.omega_0;
        fit_params.theta_sun = fit_params.theta_0 + fit_params.v_sun;
        // Save the new parameters
        self.fit_params = Some(fit_params);
        // // Prepare arrays for the confidence intervals
        // let mut fit_params_ep = vec![0.; self.objects.len()];
        // let mut fit_params_em = vec![0.; self.objects.len()];
        // // Define the confidence intervals
        // izip!(&mut fit_params_ep, &mut fit_params_em)
        //     .enumerate()
        //     .try_for_each(|(i, (fit_param_ep, fit_param_em))| -> Result<()> {
        //         let param = best_p[i];
        //
        //         let tolerance = 1e-11;
        //         let max_iters = 100;
        //
        //         // Find a root to the right
        //         {
        //             let problem = RootsProblem {
        //                 i,
        //                 best_l_1,
        //                 p_0: &p_0,
        //                 objects: &self.objects,
        //                 fit_params: &fit_params_clone,
        //                 rng: &rng,
        //             };
        //
        //             let min = param;
        //             let max = param + 1.;
        //             let solver = BrentRoot::new(min, max, tolerance);
        //
        //             let res = Executor::new(problem, solver)
        //                 .configure(|state| state.param(param).max_iters(max_iters))
        //                 .run()
        //                 .with_context(|| "Couldn't find a root to the right")?;
        //
        //             let param_p = *res.state().get_best_param().unwrap();
        //             *fit_param_ep = param_p - param;
        //         };
        //
        //         // Find a root to the left
        //         {
        //             let problem = RootsProblem {
        //                 i,
        //                 best_l_1,
        //                 p_0: &p_0,
        //                 objects: &self.objects,
        //                 fit_params: &fit_params_clone,
        //                 rng: &rng,
        //             };
        //
        //             let min = param - 1.;
        //             let max = param;
        //             let solver = BrentRoot::new(min, max, tolerance);
        //
        //             let res = Executor::new(problem, solver)
        //                 .configure(|state| state.param(param).max_iters(max_iters))
        //                 .run()
        //                 .with_context(|| "Couldn't find a root to the left")?;
        //
        //             let param_l = *res.state().get_best_param().unwrap();
        //             *fit_param_em = param - param_l;
        //         };
        //
        //         Ok(())
        //     })
        //     .with_context(|| "Couldn't define the confidence intervals")?;
        Ok(())
    }
}

// #[allow(clippy::missing_docs_in_private_items)]
// struct RootsProblem<'a, F, R> {
//     i: usize,
//     best_l_1: F,
//     p_0: &'a [F; 9],
//     objects: &'a Objects<F>,
//     fit_params: &'a Params<F>,
//     rng: &'a R,
// }
//
// impl<'a, F, R> CostFunction for RootsProblem<'a, F, R>
// where
//     F: Float + Debug + Default + Display + SampleUniform + Sync + Send,
//     StandardNormal: Distribution<F>,
//     R: Rng + SeedableRng + Clone,
// {
//     type Param = F;
//     type Output = F;
//
//     #[allow(clippy::indexing_slicing)]
//     #[allow(clippy::unwrap_in_result)]
//     #[allow(clippy::unwrap_used)]
//     #[replace_float_literals(F::from(literal).unwrap())]
//     fn cost(&self, param: &Self::Param) -> Result<Self::Output> {
//         let i = self.i;
//         let mut par_pairs = vec![(0., 0., 0.); self.objects.len()];
//         let mut objects = self.objects.clone();
//         let mut fit_params = self.fit_params.clone();
//         let mut rng = (*self.rng).clone();
//         // Prepare a new point
//         let mut new_p = [F::zero(); 9];
//         new_p[i] = *param;
//         // Redefine the target function
//         let f = |subset_p: &Point<F, 8>| -> Result<F> {
//             // Update the new point
//             let mut shift = 0;
//             for j in 0..subset_p.len() {
//                 if j == i {
//                     shift = 1;
//                 }
//                 new_p[j + shift] = subset_p[j];
//             }
//             // Update the parameters
//             fit_params.update_with(&new_p);
//             // Compute the value
//             compute_l_1(&mut objects, &fit_params, &mut par_pairs)
//         };
//         // Remove the initial value of the frozen parameter
//         let mut new_p_0 = [F::zero(); 8];
//         {
//             let mut shift = 0;
//             for j in 0..self.p_0.len() {
//                 if j == i {
//                     shift = 1;
//                     continue;
//                 }
//                 new_p_0[j - shift] = self.p_0[j];
//             }
//         }
//         // Find the global minimum
//         let (subset_best_l_1, _) = SA {
//             f,
//             p_0: &new_p_0,
//             t_0: 100.0,
//             t_min: 1.0,
//             bounds: &[
//                 F::zero()..F::infinity(),
//                 F::zero()..F::infinity(),
//                 F::zero()..F::infinity(),
//                 F::zero()..F::infinity(),
//                 F::zero()..F::infinity(),
//                 F::zero()..F::infinity(),
//                 F::zero()..F::infinity(),
//                 F::zero()..F::infinity(),
//             ],
//             apf: &APF::Metropolis,
//             neighbour: &NeighbourMethod::Custom {
//                 f: Box::new(|p, bounds, inner_rng| -> Result<Point<F, 8>> {
//                     // Get a vector of standard deviations
//                     let stds = Params::stds();
//                     let mut new_stds = [F::zero(); 8];
//                     {
//                         let mut shift = 0;
//                         for j in 0..stds.len() {
//                             if j == i {
//                                 shift = 1;
//                                 continue;
//                             }
//                             new_stds[j - shift] = stds[j];
//                         }
//                     }
//                     // Prepare a new point
//                     let mut inner_new_p = [F::zero(); 8];
//                     // Generate a new point
//                     izip!(&mut inner_new_p, p, bounds).enumerate().for_each(
//                         |(j, (new_c, &c, r))| {
//                             // Create a normal distribution around the current coordinate
//                             let d = Normal::new(c, stds[j]).unwrap();
//                             // Sample from this distribution
//                             let mut s = d.sample(inner_rng);
//                             // If the result is not in the range, repeat until it is
//                             while !r.contains(&s) {
//                                 s = d.sample(inner_rng);
//                             }
//                             // Save the new coordinate
//                             *new_c = F::from(s).unwrap();
//                         },
//                     );
//                     Ok(inner_new_p)
//                 }),
//             },
//             schedule: &Schedule::Fast,
//             status: &mut Status::None,
//             rng: &mut rng,
//         }
//         .findmin()
//         .with_context(|| "Couldn't find the global minimum")?;
//         Ok(subset_best_l_1 - self.best_l_1 - 0.5)
//     }
// }
