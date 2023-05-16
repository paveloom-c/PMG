//! Inner optimization problem

use super::rotcurve::compute_rot_curve_series;
use super::{Model, Object, Params};
use crate::utils;

use core::fmt::Debug;
use core::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Result;
use argmin::core::CostFunction;
use indoc::indoc;
use itertools::izip;
use num::Float;
use numeric_literals::replace_float_literals;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

/// A triple of the discrepancy
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Default, Clone)]
pub struct Triple<F> {
    pub observed: F,
    pub model: F,
    pub error: F,
}

/// Triples of the discrepancies
pub type Triples<F> = Vec<Triple<F>>;

/// A problem for the inner optimization
#[allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct InnerOptimizationProblem<'a, F> {
    pub l: F,
    pub b: F,
    pub v_sun: F,
    pub v_r_sun: F,
    pub v_r: F,
    pub v_r_error: F,
    pub mu_l_cos_b: F,
    pub mu_l_cos_b_error: F,
    pub mu_b: F,
    pub mu_b_error: F,
    pub par: F,
    pub par_e: F,
    pub fit_params: &'a Params<F>,
}

impl<'a, F> InnerOptimizationProblem<'a, F> {
    /// Compute the discrepancies
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::unwrap_used)]
    pub fn compute_triples(&self, par_r: F) -> Triples<F>
    where
        F: Float + Debug + Default,
    {
        // Unpack the problem
        let Self {
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
        } = *self;
        // Create an object for the reduced values
        let mut object_r = Object {
            l: Some(l),
            b: Some(b),
            par: Some(par_r),
            ..Default::default()
        };
        // Compute the values
        object_r.compute_r_h_nominal();
        object_r.compute_r_g_nominal(fit_params);
        // Unpack the data
        let r_h_r = object_r.r_h.unwrap();
        let r_g_r = object_r.r_g.unwrap();
        // Unpack the parameters
        let Params {
            r_0,
            omega_0,
            u_sun,
            w_sun,
            k,
            ..
        } = *fit_params;
        // Compute the sines and cosines of the longitude and latitude
        let sin_l = l.sin();
        let sin_b = b.sin();
        let cos_l = l.cos();
        let cos_b = b.cos();
        // Compute the difference between the Galactocentric distances
        let delta_r_g = r_g_r - r_0;
        // Compute the sum of the terms in the series of the rotation curve
        let rot_curve_series = compute_rot_curve_series(delta_r_g, fit_params);
        // Compute the full model velocity
        let v_r_rot = rot_curve_series * r_0 / r_g_r * sin_l * cos_b;
        let v_r_mod = v_r_rot + v_r_sun;
        // Compute the model proper motion in longitude
        let mu_l_cos_b_rot =
            rot_curve_series * (r_0 * cos_l / r_h_r - cos_b) / r_g_r - omega_0 * cos_b;
        let mu_l_cos_b_sun = (u_sun * sin_l - v_sun * cos_l) / r_h_r;
        let mu_l_cos_b_mod = (mu_l_cos_b_rot + mu_l_cos_b_sun) / k;
        // Compute the model proper motion in latitude
        let mu_b_rot = -rot_curve_series * r_0 / r_g_r / r_h_r * sin_l * sin_b;
        let mu_b_sun = (u_sun * cos_l * sin_b + v_sun * sin_l * sin_b - w_sun * cos_b) / r_h_r;
        let mu_b_mod = (mu_b_rot + mu_b_sun) / k;
        // Return the triples
        vec![
            Triple {
                observed: v_r,
                model: v_r_mod,
                error: v_r_error,
            },
            Triple {
                observed: mu_l_cos_b,
                model: mu_l_cos_b_mod,
                error: mu_l_cos_b_error,
            },
            Triple {
                observed: mu_b,
                model: mu_b_mod,
                error: mu_b_error,
            },
            Triple {
                observed: par,
                model: par_r,
                error: par_e,
            },
        ]
    }
}

impl<'a, F> CostFunction for InnerOptimizationProblem<'a, F>
where
    F: Float + Debug + Default,
{
    type Param = F;
    type Output = F;

    // Find the reduced parallax
    fn cost(&self, p: &Self::Param) -> Result<Self::Output> {
        let par_r = *p;
        // Compute the discrepancies
        let triples = self.compute_triples(par_r);
        // Compute the sum
        let mut sum = F::zero();
        for triple in triples {
            // We don't use the function below here because there is a slight difference in the
            // squared values, which sometimes leads to huge difference in the results
            sum = sum + (triple.observed - triple.model).powi(2) / triple.error.powi(2);
        }
        Ok(sum)
    }
}

/// Compute the relative discrepancy from a triplet
pub fn compute_relative_discrepancy<F>(triple: &Triple<F>) -> F
where
    F: Float + Debug,
{
    (triple.observed - triple.model).abs() / triple.error
}

/// Prepare the inner problem
#[allow(clippy::similar_names)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
pub fn prepare_inner_problem<'a, F>(
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
    let mut d_v_r = v_r_e.powi(2) + d_v_r_natural;
    let mut d_mu_l_cos_b = d_mu_l_cos_b_observed + d_mu_l_cos_b_natural;
    let mut d_mu_b = d_mu_b_observed + d_mu_b_natural;
    // We account for the uncertainty in transferring the
    // maser motions to that of the central star by adding
    // an error term here for non-Reid objects.
    //
    // See Reid et al. (2019)
    if !object.from_reid.as_ref().unwrap() {
        let term = 10.;
        d_v_r = d_v_r + term.powi(2);
        d_mu_l_cos_b = d_mu_l_cos_b + term.powi(2) / delim;
        d_mu_b = d_mu_b + term.powi(2) / delim;
    }
    let v_r_error = F::sqrt(d_v_r);
    let mu_l_cos_b_error = F::sqrt(d_mu_l_cos_b);
    let mu_b_error = F::sqrt(d_mu_b);
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

impl<F> Model<F> {
    /// Compute the profiles of the inner target function. Also, find those objects
    /// that have profiles with multiple local minima and output their coordinates
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::pattern_type_mismatch)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn analyze_inner_profiles(&self) -> Result<()>
    where
        F: Float + Debug + Default + Display + Sync + Send,
    {
        let fit_params = self.fit_params.as_ref().unwrap();

        let inner_profiles_dir = &self.output_dir.join("Inner profiles");
        std::fs::create_dir_all(inner_profiles_dir)?;

        let mut odd_objects = vec![false; self.objects.borrow().len()];

        self.objects
            .borrow()
            .par_iter()
            .enumerate()
            .zip(&mut odd_objects)
            .try_for_each(|((i, object), odd_object)| -> Result<()> {
                if object.outlier {
                    return Ok(());
                }

                let par = object.par.unwrap();
                let par_e = object.par_e.unwrap();

                let problem = prepare_inner_problem(object, fit_params);

                let n_points = 1000;
                let start = F::max(F::epsilon(), par - 9. * par_e);
                let end = par + 9. * par_e;
                let h = (end - start) / F::from(n_points).unwrap();

                let inner_profile_path = inner_profiles_dir.join(format!("{}.dat", i + 1));
                let inner_profile_file = File::create(inner_profile_path)?;
                let mut inner_profile_writer = BufWriter::new(inner_profile_file);

                writeln!(
                    &mut inner_profile_writer,
                    "# Profile of the inner target function\npar_r sum"
                )?;

                let diff_epsilon = F::sqrt(F::epsilon());
                let mut subinterval_start_diff =
                    utils::central_diff(start, &|x| problem.cost(&x).unwrap(), diff_epsilon);

                let mut minima_count = 0;
                for j in 0..=n_points {
                    let par_r = start + F::from(j).unwrap() * h;
                    let sum = problem.cost(&par_r)?;

                    let subinterval_end_diff =
                        utils::central_diff(par_r, &|x| problem.cost(&x).unwrap(), diff_epsilon);

                    writeln!(inner_profile_writer, "{par_r} {sum}")?;

                    if subinterval_start_diff < 0. && subinterval_end_diff > 0. {
                        minima_count += 1;
                    }

                    subinterval_start_diff = subinterval_end_diff;
                }

                if minima_count > 1 {
                    *odd_object = true;
                }

                Ok(())
            })?;

        let odd_objects_path = &self.output_dir.join("odd_objects.dat");
        let odd_objects_file = File::create(odd_objects_path)?;
        let mut odd_objects_writer = BufWriter::new(odd_objects_file);

        writeln!(
            &mut odd_objects_writer,
            indoc!(
                "
                # Coordinates of the objects that have more than one local
                # minima in their profile of the inner target function
                i name source X X_p X_m Y Y_p Y_m"
            ),
        )?;

        for ((i, object), odd_object) in
            izip!(self.objects.borrow().iter().enumerate(), &odd_objects)
        {
            if *odd_object {
                writeln!(
                    odd_objects_writer,
                    "{} \"{}\" \"{}\" {} {} {} {} {} {}",
                    i + 1,
                    object.name.as_ref().unwrap(),
                    object.source.as_ref().unwrap(),
                    object.x.as_ref().unwrap(),
                    object.x_p.as_ref().unwrap(),
                    object.x_m.as_ref().unwrap(),
                    object.y.as_ref().unwrap(),
                    object.y_p.as_ref().unwrap(),
                    object.y_m.as_ref().unwrap(),
                )?;
            }
        }

        Ok(())
    }
}
