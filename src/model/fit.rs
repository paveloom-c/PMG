//! Fit the model of the Galaxy to the data

use super::{Measurement, Model, Object, Params};

use core::cell::RefCell;
use core::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::{anyhow, Context, Result};
use indoc::formatdoc;
use itertools::izip;
use num::Float;
use numeric_literals::replace_float_literals;
use rand::distributions::uniform::SampleUniform;
use rand::prelude::Distribution;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use rand_distr::{Normal, StandardNormal};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use simulated_annealing::{NeighbourMethod, Point, Schedule, Status, APF, SA};

impl<F> Model<F>
where
    F: Float + Debug + Default + Display + SampleUniform + Sync + Send,
    StandardNormal: Distribution<F>,
{
    /// Try to fit the model of the Galaxy to the data
    #[allow(clippy::as_conversions)]
    #[allow(clippy::non_ascii_literal)]
    #[allow(clippy::shadow_unrelated)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::use_debug)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub(super) fn try_fit_from(&mut self) -> Result<()> {
        // Prepare a log file
        let log_file = File::create(self.output_dir.join("fit.log"))
            .with_context(|| "Couldn't create a file")?;
        // Prepare a buffered writer
        let wtr = RefCell::new(BufWriter::new(log_file));
        // Prepare storage for new parameters
        let mut params = self.params.clone();
        // Get the initial point in the parameter space
        let p_0 = params.to_point();
        // Prepare the spherical coordinates
        // (we don't optimize the angle parameters here)
        self.objects.iter_mut().for_each(|object| {
            object.compute_l_b(&params);
            object.compute_r_h();
        });
        // Prepare storage for the results of computing the reduced parallax
        let mut par_pairs = vec![(0., 0., 0.); self.objects.len()];
        // A closure to compute the parameterized part of the negative log likelihood function of the model
        let f = |f_p: &Point<F, 9>| -> Result<F> {
            // Update the parameters
            params.update_with(f_p);
            // Compute the new value of the function
            //
            // We compute many values manually here since
            // we don't need the numeric error propagation
            let l_1 = self
                .objects
                .par_iter_mut()
                .zip(par_pairs.par_iter_mut())
                .enumerate()
                .try_fold_with(F::zero(), |acc, (i, (object, par_pair))| -> Result<F> {
                    // Prepare a random number generator with a specific stream
                    let mut rng = ChaCha8Rng::seed_from_u64(1);
                    rng.set_stream(i as u64 + 1);
                    // Compute some values
                    object.compute_r_g(&params);
                    object.compute_mu_l_mu_b(&params);
                    // Unpack the data
                    let v_lsr = object.v_lsr.as_ref().unwrap();
                    let par = object.par.as_ref().unwrap();
                    let r_h = object.r_h.as_ref().unwrap();
                    let l = object.l.unwrap();
                    let b = object.b.unwrap();
                    let mu_l = object.mu_l.unwrap();
                    let mu_b = object.mu_b.unwrap();
                    let r_g = object.r_g.as_ref().unwrap();
                    // Compute the sines and cosines of the longitude and latitude
                    let sin_l = l.sin();
                    let sin_b = b.sin();
                    let cos_l = l.cos();
                    let cos_b = b.cos();
                    // Compute their squares
                    let sin_l_sq = sin_l.powi(2);
                    let sin_b_sq = sin_b.powi(2);
                    let cos_l_sq = cos_l.powi(2);
                    let cos_b_sq = cos_b.powi(2);
                    // Compute the normal dispersions
                    let sigma_r_sq = params.sigma_r.powi(2);
                    let sigma_theta_sq = params.sigma_theta.powi(2);
                    let sigma_z_sq = params.sigma_z.powi(2);
                    // Compute the squares of the sines and cosines of the Galactocentric longitude
                    let sin_lambda_sq = ((r_h.v * cos_b * sin_l) / r_g.v).powi(2);
                    let cos_lambda_sq = ((params.r_0 - r_h.v * cos_b * cos_l) / r_g.v).powi(2);
                    // Compute auxiliary sums of the squares of the sines and cosines
                    let sum_1 = cos_lambda_sq * cos_l_sq + sin_lambda_sq * sin_l_sq;
                    let sum_2 = sin_lambda_sq * cos_l_sq + cos_lambda_sq * sin_l_sq;
                    // Compute the model-dependent dispersions
                    let sigma_v_r_star_sq = sigma_r_sq * sum_1 * cos_b_sq
                        + sigma_theta_sq * sum_2 * cos_b_sq
                        + sigma_z_sq * sin_b_sq;
                    let sigma_v_l_star_sq = sigma_r_sq * sum_2 + sigma_theta_sq * sum_1;
                    let sigma_v_b_star_sq = sigma_r_sq * sum_1 * sin_b_sq
                        + sigma_theta_sq * sum_2 * sin_b_sq
                        + sigma_z_sq * cos_b_sq;
                    // Compute the dispersions of the observed proper motions
                    let (sigma_mu_l_cos_b_sq, sigma_mu_b_sq) = object.compute_e_mu_l_mu_b(&params);
                    // Compute the full dispersions
                    let delim = params.k.powi(2) * r_h.v.powi(2);
                    let d_v_r = v_lsr.e_p.powi(2) + sigma_v_r_star_sq;
                    let d_mu_l_cos_b = sigma_mu_l_cos_b_sq + sigma_v_l_star_sq / delim;
                    let d_mu_b = sigma_mu_b_sq + sigma_v_b_star_sq / delim;
                    let d_par = par.e_p.powi(2);
                    // Compute the constant part of the model velocity
                    let v_r_sun = -params.u_sun_standard * cos_l * cos_b
                        - params.v_sun_standard * sin_l * cos_b
                        - params.w_sun_standard * sin_b;
                    // Prepare a closure for finding the reduced parallax
                    let g = |g_p: &Point<F, 1>| -> Result<F> {
                        // Create an object for reduced values
                        let mut object_r = Object {
                            l: Some(l),
                            b: Some(b),
                            par: Some(Measurement {
                                v: g_p[0],
                                ..Default::default()
                            }),
                            ..Default::default()
                        };
                        // Compute the values
                        object_r.compute_r_h_nominal();
                        object_r.compute_r_g_nominal(&params);
                        // Unpack the data
                        let par_r = object_r.par.unwrap().v;
                        let r_h_r = object_r.r_h.unwrap().v;
                        let r_g_r = object_r.r_g.unwrap().v;
                        // Compute the difference between the Galactocentric distances
                        let delta_r = r_g_r - params.r_0;
                        // Compute the sum of the terms in the series of the rotation curve
                        let rot_curve_series = 2. * params.a * delta_r;
                        // Compute the full model velocity
                        let v_r_mod =
                            -rot_curve_series * params.r_0 * sin_l * cos_b / r_g_r + v_r_sun;
                        // Compute the model proper motion in longitude
                        let mu_l_cos_b_mod =
                            (-rot_curve_series * (params.r_0 * cos_l / r_h_r - cos_b) / r_g_r
                                - params.omega_0 * cos_b
                                + (params.u_sun_standard * sin_l - params.v_sun_standard * cos_l)
                                    / r_h_r)
                                / params.k
                                * cos_b;
                        // Compute the model proper motion in latitude
                        let mu_b_mod =
                            (rot_curve_series * params.r_0 * sin_l * sin_b / r_h_r / r_g_r
                                + (params.u_sun_standard * cos_l * sin_b
                                    + params.v_sun_standard * sin_l * sin_b
                                    - params.w_sun_standard * cos_b)
                                    / r_h_r)
                                / params.k;
                        // Compute the weighted sum of squared differences
                        let sum = (v_lsr.v - v_r_mod).powi(2) / d_v_r
                            + (mu_l * cos_b - mu_l_cos_b_mod).powi(2) / d_mu_l_cos_b
                            + (mu_b - mu_b_mod).powi(2) / d_mu_b
                            + (par.v - par_r).powi(2) / d_par;
                        // Return it as the result
                        Ok(sum)
                    };
                    // Find the global minimum
                    let (sum_min, par_r) = SA {
                        f: g,
                        p_0: &[par.v],
                        t_0: 100_000.0,
                        t_min: 1.0,
                        bounds: &[F::zero()..F::infinity()],
                        apf: &APF::Metropolis,
                        neighbour: &NeighbourMethod::Normal { sd: par.e_p },
                        schedule: &Schedule::Fast,
                        status: &mut Status::None,
                        rng: &mut rng,
                    }
                    .findmin()?;
                    // Compute the final sum for this object
                    let res = F::ln(F::sqrt(d_v_r))
                        + F::ln(F::sqrt(d_mu_l_cos_b))
                        + F::ln(F::sqrt(d_mu_b))
                        + 0.5 * sum_min;
                    // Save the results
                    *par_pair = (par.v, par.e_p, par_r[0]);
                    // Add to the general sum
                    Ok(acc + res)
                })
                // Parallel fold returns an iterator over folds from
                // different threads. We sum those to get the final results
                .reduce(|| Ok(F::zero()), |a, b| Ok(a? + b?));
            // Write the result to the buffer
            for (i, &(par_v, par_e_p, par_r)) in par_pairs.iter().enumerate() {
                writeln!(
                    wtr.borrow_mut(),
                    "{i}: par: {par_v} ± {par_e_p} -> par_r: {par_r}",
                )
                .ok();
            }
            l_1
        };
        // Find the global minimum
        let (_, p) = SA {
            f,
            p_0: &p_0,
            t_0: 100_000.0,
            t_min: 1.0,
            bounds: &Params::bounds(),
            apf: &APF::Metropolis,
            neighbour: &NeighbourMethod::Custom {
                f: |p, bounds, rng| -> Result<Point<F, 9>> {
                    // Prepare a vector of standard deviations
                    let stds = [0.25, 0.25, 0.5, 0.05, 0.05, 0.05, 1., 1., 1.];
                    // Prepare a new point
                    let mut new_p = [F::zero(); 9];
                    // Generate a new point
                    izip!(&mut new_p, p, bounds)
                        .enumerate()
                        .try_for_each(|(i, (new_c, &c, r))| -> Result<()> {
                            // Create a normal distribution around the current coordinate
                            #[allow(clippy::indexing_slicing)]
                            let d = Normal::new(c, stds[i])
                                .with_context(|| "Couldn't create a normal distribution")?;
                            // Sample from this distribution
                            let mut s = d.sample(rng);
                            // If the result is not in the range, repeat until it is
                            while !r.contains(&s) {
                                s = d.sample(rng);
                            }
                            // Save the new coordinate
                            *new_c = F::from(s).ok_or_else(|| {
                                anyhow!("Couldn't cast a value to a floating-point number")
                            })?;
                            Ok(())
                        })
                        .with_context(|| "Couldn't generate a new point")?;
                    Ok(new_p)
                },
            },
            schedule: &Schedule::Exponential { gamma: 0.95 },
            status: &mut Status::Custom {
                f: Box::new(|k, t, f, p, best_f, best_p| {
                    writeln!(
                        wtr.borrow_mut(),
                        "{}",
                        formatdoc!(
                            "
                            k: {k}
                            t: {t}
                                            {:>10} initial {:>10} current {:>13} best
                                       L_1: {:>16} — {f:>18} {best_f:>18}
                                       r_0: {i_0:>18.15} {p_0:>18.15} {best_p_0:>18.15}
                                   omega_0: {i_1:>18.15} {p_1:>18.15} {best_p_1:>18.15}
                                         a: {i_2:>18.15} {p_2:>18.15} {best_p_2:>18.15}
                            u_sun_standard: {i_3:>18.15} {p_3:>18.15} {best_p_3:>18.15}
                            v_sun_standard: {i_4:>18.15} {p_4:>18.15} {best_p_4:>18.15}
                            w_sun_standard: {i_5:>18.15} {p_5:>18.15} {best_p_5:>18.15}
                                   sigma_r: {i_6:>18.15} {p_6:>18.15} {best_p_6:>18.15}
                               sigma_theta: {i_7:>18.15} {p_7:>18.15} {best_p_7:>18.15}
                                   sigma_z: {i_8:>18.15} {p_8:>18.15} {best_p_8:>18.15}
                            ",
                            "",
                            "",
                            "",
                            "",
                            i_0 = self.params.r_0,
                            i_1 = self.params.omega_0,
                            i_2 = self.params.a,
                            i_3 = self.params.u_sun_standard,
                            i_4 = self.params.v_sun_standard,
                            i_5 = self.params.w_sun_standard,
                            i_6 = self.params.sigma_r,
                            i_7 = self.params.sigma_theta,
                            i_8 = self.params.sigma_z,
                            p_0 = p[0],
                            p_1 = p[1],
                            p_2 = p[2],
                            p_3 = p[3],
                            p_4 = p[4],
                            p_5 = p[5],
                            p_6 = p[6],
                            p_7 = p[7],
                            p_8 = p[8],
                            best_p_0 = best_p[0],
                            best_p_1 = best_p[1],
                            best_p_2 = best_p[2],
                            best_p_3 = best_p[3],
                            best_p_4 = best_p[4],
                            best_p_5 = best_p[5],
                            best_p_6 = best_p[6],
                            best_p_7 = best_p[7],
                            best_p_8 = best_p[8],
                        ),
                    )
                    .ok();
                }),
            },
            // Same seed as above, but the stream is 0
            rng: &mut ChaCha8Rng::seed_from_u64(1),
        }
        .findmin()
        .with_context(|| "Couldn't find the global minimum")?;
        // Update the parameters one more time
        params.update_with(&p);
        // Save the new parameters
        self.fit_params = Some(params);
        Ok(())
    }
}
