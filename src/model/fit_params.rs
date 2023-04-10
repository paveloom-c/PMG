//! Fit the model of the Galaxy to the data

use super::{Model, Object, Objects, Params};

use core::cell::RefCell;
use core::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::{Context, Result};
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

/// Random number generator seed
const RNG_SEED: u64 = 1;

/// Compute the parameterized part of the negative log likelihood function of the model
#[allow(clippy::as_conversions)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::similar_names)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::unwrap_in_result)]
#[allow(clippy::unwrap_used)]
#[replace_float_literals(F::from(literal).unwrap())]
fn compute_l_1<F>(
    objects: &mut Objects<F>,
    fit_params: &Params<F>,
    par_pairs: &mut Vec<(F, F, F)>,
) -> Result<F>
where
    F: Float + Debug + Default + Display + SampleUniform + Sync + Send,
    StandardNormal: Distribution<F>,
{
    // Compute the new value of the function
    objects
        .par_iter_mut()
        .zip(par_pairs.par_iter_mut())
        .enumerate()
        .try_fold_with(F::zero(), |acc, (i, (object, par_pair))| -> Result<F> {
            // Prepare a random number generator with a specific stream
            let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED);
            rng.set_stream(i as u64 + 1);
            // Compute some values
            object.compute_r_g(fit_params);
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
            let r_0 = fit_params.r_0;
            let omega_0 = fit_params.omega_0;
            let a = fit_params.a;
            let u_sun = fit_params.u_sun;
            let theta_sun = fit_params.theta_sun;
            let w_sun = fit_params.w_sun;
            let sigma_r = fit_params.sigma_r;
            let sigma_theta = fit_params.sigma_theta;
            let sigma_z = fit_params.sigma_z;
            let k = fit_params.k;
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
            let d_r = sigma_r.powi(2);
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
                d_r * cos_phi_sq * cos_b_sq + d_theta * sin_phi_sq * cos_l_sq + d_z * sin_b_sq;
            let d_v_l_natural = d_r * sin_phi_sq + d_theta * cos_phi_sq;
            let d_v_b_natural =
                d_r * cos_phi_sq * sin_b_sq + d_theta * sin_phi_sq * sin_b_sq + d_z * cos_b_sq;
            let delim = k.powi(2) * r_h.powi(2);
            let d_mu_l_cos_b_natural = d_v_l_natural / delim;
            let d_mu_b_natural = d_v_b_natural / delim;
            // Compute the dispersions of the observed proper motions
            let (d_mu_l_cos_b_observed, d_mu_b_observed) =
                object.compute_d_mu_l_cos_b_mu_b(fit_params);
            // Compute the full dispersions
            let d_v_r = v_r_e.powi(2) + d_v_r_natural;
            let d_mu_l_cos_b = d_mu_l_cos_b_observed + d_mu_l_cos_b_natural;
            let d_mu_b = d_mu_b_observed + d_mu_b_natural;
            let d_par = par_e.powi(2);
            // Compute the peculiar motion of the Sun toward l = 90 degrees (km/s)
            let v_sun = theta_sun - r_0 * omega_0;
            // Compute the constant part of the model velocity
            let v_r_sun = -u_sun * cos_l * cos_b - v_sun * sin_l * cos_b - w_sun * sin_b;
            // Prepare a closure for finding the reduced parallax
            let g = |g_p: &Point<F, 1>| -> Result<F> {
                let par_r = g_p[0];
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
                // Compute the difference between the Galactocentric distances
                let delta_r = r_g_r - r_0;
                // Compute the sum of the terms in the series of the rotation curve
                let rot_curve_series = -2. * a * delta_r;
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
                let mu_b_sun =
                    (u_sun * cos_l * sin_b + v_sun * sin_l * sin_b - w_sun * cos_b) / r_h_r;
                let mu_b_mod = (mu_b_rot + mu_b_sun) / k;
                // Compute the weighted sum of squared differences
                let sum = (v_r - v_r_mod).powi(2) / d_v_r
                    + (mu_l_cos_b - mu_l_cos_b_mod).powi(2) / d_mu_l_cos_b
                    + (mu_b - mu_b_mod).powi(2) / d_mu_b
                    + (par - par_r).powi(2) / d_par;
                // Return it as the result
                Ok(sum)
            };
            // Find the global minimum
            let (sum_min, par_r) = SA {
                f: g,
                p_0: &[par],
                t_0: 100.0,
                t_min: 1.0,
                bounds: &[F::zero()..F::infinity()],
                apf: &APF::Custom {
                    f: |diff, _, _, _| {
                        // Always go downhill
                        diff <= F::zero()
                    },
                },
                neighbour: &NeighbourMethod::Normal { sd: par_e },
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
            *par_pair = (par, par_e, par_r[0]);
            // Add to the general sum
            Ok(acc + res)
        })
        // Parallel fold returns an iterator over folds from
        // different threads. We sum those to get the final results
        .reduce(|| Ok(F::zero()), |a, b| Ok(a? + b?))
}

impl<F> Model<F>
where
    F: Float + Debug + Default + Display + SampleUniform + Sync + Send,
    StandardNormal: Distribution<F>,
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
    pub(super) fn try_fit_from(&mut self) -> Result<()> {
        // Prepare a log file
        let log_file = File::create(self.output_dir.join("fit.log"))
            .with_context(|| "Couldn't create a file")?;
        // Prepare a buffered writer
        let wtr = RefCell::new(BufWriter::new(log_file));
        // Prepare storage for new parameters
        let mut fit_params = self.params.clone();
        // Get the initial point in the parameter space
        let p_0 = fit_params.to_point();
        // Compute some of the values that don't
        // depend on the parameters being optimized
        self.objects.iter_mut().for_each(|object| {
            object.compute_l_b(&fit_params);
            object.compute_v_r(&fit_params);
            object.compute_r_h();
            object.compute_mu_l_cos_b_mu_b(&fit_params);
        });
        // Prepare storage for the results of computing the reduced parallax
        let mut par_pairs = vec![(0., 0., 0.); self.objects.len()];
        // A closure to compute the parameterized part of
        // the negative log likelihood function of the model
        let f = |f_p: &Point<F, 9>| -> Result<F> {
            // Update the parameters
            fit_params.update_with(f_p);
            // Compute the value
            let l_1 = compute_l_1(&mut self.objects, &fit_params, &mut par_pairs);
            // Write the results of finding the reduced parallax to the buffer
            for (i, &(par, par_e, par_r)) in par_pairs.iter().enumerate() {
                writeln!(
                    wtr.borrow_mut(),
                    "{i}: par: {par} ± {par_e} -> par_r: {par_r}",
                )
                .ok();
            }
            // Return the value
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
                    // Get a vector of standard deviations
                    let stds = Params::stds();
                    // Prepare a new point
                    let mut new_p = [F::zero(); 9];
                    // Generate a new point
                    izip!(&mut new_p, p, bounds)
                        .enumerate()
                        .for_each(|(i, (new_c, &c, r))| {
                            // Create a normal distribution around the current coordinate
                            let d = Normal::new(c, stds[i]).unwrap();
                            // Sample from this distribution
                            let mut s = d.sample(rng);
                            // If the result is not in the range, repeat until it is
                            while !r.contains(&s) {
                                s = d.sample(rng);
                            }
                            // Save the new coordinate
                            *new_c = F::from(s).unwrap();
                        });
                    Ok(new_p)
                },
            },
            schedule: &Schedule::Fast,
            status: &mut Status::Custom {
                f: Box::new(|k, t, f, p, best_f, best_p| {
                    writeln!(
                        wtr.borrow_mut(),
                        "{}",
                        formatdoc!(
                            "
                            k: {k}
                            t: {t}
                                            {:>11} initial {:>11} current {:>14} best
                                       L_1: {:>17} — {f:>19} {best_f:>19}
                                       r_0: {i_0:>19.15} {p_0:>19.15} {best_p_0:>19.15}
                                   omega_0: {i_1:>19.15} {p_1:>19.15} {best_p_1:>19.15}
                                         a: {i_2:>19.15} {p_2:>19.15} {best_p_2:>19.15}
                                     u_sun: {i_3:>19.15} {p_3:>19.15} {best_p_3:>19.15}
                                 theta_sun: {i_4:>19.15} {p_4:>19.15} {best_p_4:>19.15}
                                     w_sun: {i_5:>19.15} {p_5:>19.15} {best_p_5:>19.15}
                                   sigma_r: {i_6:>19.15} {p_6:>19.15} {best_p_6:>19.15}
                               sigma_theta: {i_7:>19.15} {p_7:>19.15} {best_p_7:>19.15}
                                   sigma_z: {i_8:>19.15} {p_8:>19.15} {best_p_8:>19.15}
                            ",
                            "",
                            "",
                            "",
                            "",
                            i_0 = self.params.r_0,
                            i_1 = self.params.omega_0,
                            i_2 = self.params.a,
                            i_3 = self.params.u_sun,
                            i_4 = self.params.theta_sun,
                            i_5 = self.params.w_sun,
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
            rng: &mut ChaCha8Rng::seed_from_u64(RNG_SEED),
        }
        .findmin()
        .with_context(|| "Couldn't find the global minimum")?;
        // Update the parameters one more time
        fit_params.update_with(&p);
        // Save the new parameters
        self.fit_params = Some(fit_params);
        Ok(())
    }
}
