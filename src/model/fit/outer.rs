//! Outer optimization problem

extern crate alloc;

use super::{compute_relative_discrepancy, InnerOptimizationProblem, Triple, Triples};
use super::{Object, Objects, Params};
use crate::utils::{self, FiniteDiff};

use alloc::rc::Rc;
use argmin::solver::brent::BrentOpt;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use argmin::core::{ArgminFloat, CostFunction, Executor, Gradient, State};
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use num::Float;
use numeric_literals::replace_float_literals;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

/// A problem for the outer optimization
#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::type_complexity)]
pub struct OuterOptimizationProblem<'a, F> {
    pub objects: &'a Rc<RefCell<Objects<F>>>,
    pub params: &'a Params<F>,
    pub triples: &'a Rc<RefCell<Vec<Triples<F>>>>,
    pub output_dir: &'a PathBuf,
}

/// Type of the parameters
pub type Param<F> = Vec<F>;

/// Type of the output
pub type Output<F> = F;

/// Prepare the inner problem
#[allow(clippy::similar_names)]
#[allow(clippy::unwrap_used)]
fn prepare_inner_problem<'a, F>(
    object: &Object<F>,
    fit_params: &'a Params<F>,
) -> InnerOptimizationProblem<'a, F>
where
    F: Float + Debug + Default,
{
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
    let Params {
        r_0,
        u_sun,
        v_sun,
        w_sun,
        sigma_r_g,
        sigma_theta,
        sigma_z,
        k,
        ..
    } = *fit_params;
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
    let d_r_g = sigma_r_g.powi(2);
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
        d_r_g * cos_phi_sq * cos_b_sq + d_theta * sin_phi_sq * cos_l_sq + d_z * sin_b_sq;
    let d_v_l_natural = d_r_g * sin_phi_sq + d_theta * cos_phi_sq;
    let d_v_b_natural =
        d_r_g * cos_phi_sq * sin_b_sq + d_theta * sin_phi_sq * sin_b_sq + d_z * cos_b_sq;
    let delim = k.powi(2) * r_h.powi(2);
    let d_mu_l_cos_b_natural = d_v_l_natural / delim;
    let d_mu_b_natural = d_v_b_natural / delim;
    // Compute the dispersions of the observed proper motions
    let (d_mu_l_cos_b_observed, d_mu_b_observed) = object.compute_d_mu_l_cos_b_mu_b(fit_params);
    // Compute the full errors
    let v_r_error = F::sqrt(v_r_e.powi(2) + d_v_r_natural);
    let mu_l_cos_b_error = F::sqrt(d_mu_l_cos_b_observed + d_mu_l_cos_b_natural);
    let mu_b_error = F::sqrt(d_mu_b_observed + d_mu_b_natural);
    // Compute the constant part of the model velocity
    let v_r_sun = -u_sun * cos_l * cos_b - v_sun * sin_l * cos_b - w_sun * sin_b;
    // Define a problem of the inner optimization
    InnerOptimizationProblem {
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
    }
}

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
    pub fn inner_cost(
        &self,
        p: &Param<F>,
        update_triples: bool,
        check_par_vicinities: bool,
        with_blacklisted: bool,
    ) -> Result<Output<F>>
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
                // Skip if the object has been blacklisted
                if object.blacklisted {
                    return Ok(());
                };

                // Compute some values
                object.compute_r_g(&fit_params);
                // Unpack the data
                let par = object.par.unwrap();
                let par_e = object.par_e.unwrap();
                // Define a problem of the inner optimization
                let problem = prepare_inner_problem(object, &fit_params);
                let init_param = par;
                let solver =
                    BrentOpt::new(F::max(F::epsilon(), par - 3. * par_e), par + 3. * par_e)
                        .set_tolerance(F::sqrt(F::epsilon()), 1e-15);
                // Find the local minimum in the inner optimization
                let res = Executor::new(problem.clone(), solver)
                    .configure(|state| state.param(init_param).max_iters(1000))
                    .run()
                    .with_context(|| "Couldn't solve the inner optimization problem")?;
                let &par_r = res.state().get_best_param().unwrap();
                let best_cost = res.state().get_best_cost();

                if check_par_vicinities {
                    let n_points = 100;
                    let start = F::max(F::epsilon(), par - 3. * par_e);
                    let end = par + 3. * par_e;
                    let h = (end - start) / F::from(n_points).unwrap();

                    let mut extrema_count = 0;
                    let epsilon = F::sqrt(F::epsilon());
                    let start_diff =
                        utils::central_diff(start, &|x| problem.cost(&x).unwrap(), epsilon);
                    let mut current_signum = start_diff.signum();
                    for j in 0..=n_points {
                        let par_test = start + F::from(j).unwrap() * h;

                        let diff = utils::central_diff(
                            par_test,
                            &|x| problem.cost(&x).unwrap(),
                            F::sqrt(F::epsilon()),
                        );
                        let diff_signum = diff.signum();

                        if diff_signum * current_signum < 0. {
                            current_signum = diff_signum;
                            extrema_count += 1;
                        }
                    }

                    if extrema_count != 1 {
                        object.blacklisted = true;
                    }
                }

                // Compute the final sum for this object
                *cost = F::ln(problem.v_r_error)
                    + F::ln(problem.mu_l_cos_b_error)
                    + F::ln(problem.mu_b_error)
                    + 0.5 * best_cost;
                // Save the results
                if update_triples {
                    *triple = problem.compute_triples(par_r);
                }
                Ok(())
            })?;
        // We do the summing sequentially because
        // floating-point arithmetic is not associative
        let cost = costs.iter().copied().sum();

        if with_blacklisted {
            let blacklisted_objects: Vec<(usize, Object<F>)> = self
                .objects
                .borrow()
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_, object)| object.blacklisted)
                .collect();

            let triple = vec![Triple::<F>::default(); 4];
            let mut observed_triples_vec = vec![triple; blacklisted_objects.len()];
            let mut reduced_triples_vec = observed_triples_vec.clone();

            let blacklisted_objects_path = self.output_dir.join("Blacklisted objects");
            std::fs::create_dir_all(blacklisted_objects_path.clone())?;

            blacklisted_objects
                .par_iter()
                .zip(observed_triples_vec.par_iter_mut())
                .zip(reduced_triples_vec.par_iter_mut())
                .try_for_each(
                    |(((i, object), observed_triples), reduced_triples)| -> Result<()> {
                        // Unpack the data
                        let par = object.par.unwrap();
                        let par_e = object.par_e.unwrap();
                        // Define a problem of the inner optimization
                        let problem = prepare_inner_problem(object, &fit_params);
                        let init_param = par;
                        let solver =
                            BrentOpt::new(F::max(F::epsilon(), par - 3. * par_e), par + 3. * par_e)
                                .set_tolerance(F::sqrt(F::epsilon()), 1e-15);
                        // Find the local minimum in the inner optimization
                        let res = Executor::new(problem.clone(), solver)
                            .configure(|state| state.param(init_param).max_iters(1000))
                            .run()
                            .with_context(|| "Couldn't solve the inner optimization problem")?;
                        let &par_r = res.state().get_best_param().unwrap();

                        let n_points = 1000;
                        let start = F::max(F::epsilon(), par - 3. * par_e);
                        let end = par + 3. * par_e;
                        let h = (end - start) / F::from(n_points).unwrap();

                        // Prepare an output directory
                        let inner_profiles_path = blacklisted_objects_path.join("Inner profiles");
                        std::fs::create_dir_all(inner_profiles_path.clone())?;

                        let inner_profile_path = inner_profiles_path.join(format!("{}.dat", i + 1));
                        let inner_profile_file = File::create(inner_profile_path)?;
                        let mut inner_profile_writer = BufWriter::new(inner_profile_file);

                        writeln!(inner_profile_writer, "par_r sum")?;

                        // Compute the inner profiles
                        for j in 0..=n_points {
                            let par_test = start + F::from(j).unwrap() * h;
                            let sum = problem.cost(&par_test)?;

                            writeln!(inner_profile_writer, "{par_test} {sum}")?;
                        }

                        // Compute the discrepancies for the observed and reduced parallaxes
                        *reduced_triples = problem.compute_triples(par_r);
                        *observed_triples = problem.compute_triples(par);

                        Ok(())
                    },
                )?;

            let discrepancies_path = blacklisted_objects_path.join("discrepancies.plain");
            let discrepancies_file = File::create(discrepancies_path)?;
            let mut discrepancies_writer = BufWriter::new(discrepancies_file);

            writeln!(
                discrepancies_writer,
                "{s:>2}i{s:>9}v_r(par_r){s:>7}mu_l'(par_r){s:>8}mu_b(par_r){s:>14}par_r{s:>11}v_r(par){s:>9}mu_l'(par){s:>10}mu_b(par)",
                s = " "
            )?;

            let coords_path = blacklisted_objects_path.join("coords.dat");
            let coords_file = File::create(coords_path)?;
            let mut coords_writer = BufWriter::new(coords_file);

            writeln!(coords_writer, "X X_p X_m Y Y_p Y_m X_r Y_r")?;

            for (((i, object), reduced_triples), observed_triples) in blacklisted_objects
                .iter()
                .zip(reduced_triples_vec.iter())
                .zip(observed_triples_vec.iter())
            {
                writeln!(
                    discrepancies_writer,
                    "{:>3} {:>18.15} {:>18.15} {:>18.15} {:>18.15} {:>18.15} {:>18.15} {:>18.15}",
                    i + 1,
                    compute_relative_discrepancy(&reduced_triples[0]),
                    compute_relative_discrepancy(&reduced_triples[1]),
                    compute_relative_discrepancy(&reduced_triples[2]),
                    compute_relative_discrepancy(&reduced_triples[3]),
                    compute_relative_discrepancy(&observed_triples[0]),
                    compute_relative_discrepancy(&observed_triples[1]),
                    compute_relative_discrepancy(&observed_triples[2]),
                )?;

                let l = object.l.unwrap();
                let b = object.b.unwrap();
                let par_r = reduced_triples[3].model;

                let mut object_r = Object {
                    l: Some(l),
                    b: Some(b),
                    par: Some(par_r),
                    ..Default::default()
                };
                object_r.compute_r_h_nominal();
                object_r.compute_x_y_z_nominal();

                let x = object.x.unwrap();
                let x_p = object.x_p.unwrap();
                let x_m = object.x_m.unwrap();
                let y = object.y.unwrap();
                let y_p = object.y_p.unwrap();
                let y_m = object.y_m.unwrap();
                let x_r = object_r.x.unwrap();
                let y_r = object_r.y.unwrap();

                writeln!(coords_writer, "{x} {x_p} {x_m} {y} {y_p} {y_m} {x_r} {y_r}")?;
            }
        }

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
        self.inner_cost(p, true, false, false)
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
            &|x| self.inner_cost(x, false, false, false).unwrap(),
            F::sqrt(F::epsilon()),
        ))
    }
}
