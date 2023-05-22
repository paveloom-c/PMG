//! Profiles

extern crate alloc;

use super::io::output;
use super::{ConfidenceIntervalProblem, OuterOptimizationProblem, Triple, Triples};
use super::{Model, Params, PARAMS_N, PARAMS_NAMES};
use crate::utils::FiniteDiff;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;

use anyhow::{Context, Result};
use argmin::core::ArgminFloat;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use indoc::formatdoc;
use num::Float;
use numeric_literals::replace_float_literals;
use serde::Serialize;

/// A profile
pub type Profile<F> = Vec<ProfilePoint<F>>;

/// Type of a profile
pub enum ProfileType {
    /// One parameter fixed, the rest are free
    Conditional,
    /// All parameters are fixed
    Frozen,
}

/// A point on the profile
#[derive(Debug, Clone, Serialize)]
pub struct ProfilePoint<F> {
    /// Value of the parameter
    param: F,
    /// Value of the cost function
    cost: F,
}

/// Number of points in a profile
const POINTS_N: usize = 100;

impl<F> Model<F> {
    /// Try to compute the conditional profiles
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::print_stderr)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn try_compute_conditional_profiles(&mut self, l_stroke: usize) -> Result<()>
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
        // Get the optimized parameters as arrays
        let n = self.n.unwrap();
        let fit_params = self.fit_params.as_ref().unwrap().to_vec(n, false);
        let fit_params_ep = self.fit_params.as_ref().unwrap().to_ep_vec(n);
        let fit_params_em = self.fit_params.as_ref().unwrap().to_em_vec(n);
        // Prepare storage
        let triple = vec![Triple::<F>::default(); 4];
        let triples = Rc::new(RefCell::new(vec![triple; self.objects.borrow().len()]));

        // Compute conditional profiles (one parameter is fixed
        // and externally varied, the rest are free)
        let len = fit_params.len();
        for index in 0..len {
            // Don't compute for the sigmas or compute for the sigmas only
            if Params::<F>::compute_with_l_stroke(index, l_stroke) {
                continue;
            }

            let fit_param = fit_params[index];
            let fit_param_ep = fit_params_ep[index];
            let fit_param_em = fit_params_em[index];

            let compute_param = |x: F, _: &[F]| x;

            let profile = self.try_compute_conditional_profile(
                l_stroke,
                index,
                compute_param,
                fit_param,
                fit_param_ep,
                fit_param_em,
                &triples,
            )?;

            self.serialize_to_profile(&ProfileType::Conditional, &profile, PARAMS_NAMES[index])
                .with_context(|| "Couldn't write a conditional profile to a file")?;
        }

        // Compute the conditional profile for derived values,
        // too, by temporarily changing the parametrization
        if l_stroke == 1 {
            {
                let fit_param = self.fit_params.as_ref().unwrap().theta_0;
                let fit_param_ep = self.fit_params.as_ref().unwrap().theta_0_ep;
                let fit_param_em = self.fit_params.as_ref().unwrap().theta_0_em;

                // `omega_0` = `theta_0` / `R_0`
                let compute_param = |theta_0: F, p: &[F]| theta_0 / p[0];

                let profile = self.try_compute_conditional_profile(
                    l_stroke,
                    1,
                    compute_param,
                    fit_param,
                    fit_param_ep,
                    fit_param_em,
                    &triples,
                )?;

                self.serialize_to_profile(&ProfileType::Conditional, &profile, "theta_0")
                    .with_context(|| "Couldn't write a conditional profile to a file")?;
            }
            {
                let fit_param = self.fit_params.as_ref().unwrap().theta_1;
                let fit_param_ep = self.fit_params.as_ref().unwrap().theta_1_ep;
                let fit_param_em = self.fit_params.as_ref().unwrap().theta_1_em;

                // `omega_0` = `theta_1` + 2 * `A`
                //
                // The index is 1 and not 2 because the
                // frozen parameter (index 1) is removed
                let compute_param = |theta_1: F, p: &[F]| theta_1 + 2. * p[1];

                let profile = self.try_compute_conditional_profile(
                    l_stroke,
                    1,
                    compute_param,
                    fit_param,
                    fit_param_ep,
                    fit_param_em,
                    &triples,
                )?;

                self.serialize_to_profile(&ProfileType::Conditional, &profile, "theta_1")
                    .with_context(|| "Couldn't write a conditional profile to a file")?;
            }
            {
                let fit_param = self.fit_params.as_ref().unwrap().theta_sun;
                let fit_param_ep = self.fit_params.as_ref().unwrap().theta_sun_ep;
                let fit_param_em = self.fit_params.as_ref().unwrap().theta_sun_em;

                // `v_sun` = `theta_sun` - `R_0` * `omega_0`
                let compute_param = |theta_sun: F, p: &[F]| theta_sun - p[0] * p[1];

                let profile = self.try_compute_conditional_profile(
                    l_stroke,
                    4,
                    compute_param,
                    fit_param,
                    fit_param_ep,
                    fit_param_em,
                    &triples,
                )?;

                self.serialize_to_profile(&ProfileType::Conditional, &profile, "theta_sun")
                    .with_context(|| "Couldn't write a conditional profile to a file")?;
            }
            {
                let fit_param = self.fit_params.as_ref().unwrap().omega_sun;
                let fit_param_ep = self.fit_params.as_ref().unwrap().omega_sun_ep;
                let fit_param_em = self.fit_params.as_ref().unwrap().omega_sun_em;

                // Compute `omega_0` from `omega_sun` = `v_sun` / `R_0`
                //
                // The index is 3 and not 4 because the
                // frozen parameter (index 1) is removed
                let compute_param = |omega_sun: F, p: &[F]| omega_sun - p[3] / p[0];

                let profile = self.try_compute_conditional_profile(
                    l_stroke,
                    1,
                    compute_param,
                    fit_param,
                    fit_param_ep,
                    fit_param_em,
                    &triples,
                )?;

                self.serialize_to_profile(&ProfileType::Conditional, &profile, "omega_sun")
                    .with_context(|| "Couldn't write a conditional profile to a file")?;
            }
        }

        Ok(())
    }
    /// Try to compute a conditional profile
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::print_stderr)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn try_compute_conditional_profile<FN>(
        &mut self,
        l_stroke: usize,
        index: usize,
        compute_param: FN,
        fit_param: F,
        fit_param_ep: F,
        fit_param_em: F,
        triples: &Rc<RefCell<Vec<Triples<F>>>>,
    ) -> Result<Vec<ProfilePoint<F>>>
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
        FN: Fn(F, &[F]) -> F,
    {
        let n = self.n.unwrap();

        let coeff = 1.1;
        let start = fit_param - 3. * fit_param_em * coeff;
        let end = fit_param + 3. * fit_param_ep * coeff;
        let h = (end - start) / F::from(POINTS_N).unwrap();

        let mut profile = Vec::<ProfilePoint<F>>::with_capacity(POINTS_N);

        for j in 0..=POINTS_N {
            let param = start + F::from(j).unwrap() * h;

            let problem = ConfidenceIntervalProblem {
                disable_inner: self.disable_inner,
                l_stroke,
                n,
                index,
                best_outer_cost: F::zero(),
                objects: &self.objects,
                params: &self.params,
                compute_param: &compute_param,
                fit_params: self.fit_params.as_ref().unwrap(),
                triples: &Rc::clone(triples),
                output_dir: &self.output_dir,
            };
            let cost = problem.inner_cost(&param)?;

            profile.push(ProfilePoint { param, cost });
        }

        Ok(profile)
    }
    /// Try to compute the frozen profiles
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::print_stderr)]
    #[allow(clippy::shadow_unrelated)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn try_compute_frozen_profiles(&mut self, l_stroke: usize) -> Result<()>
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
        // Get the optimized parameters as arrays
        let n = self.n.unwrap();
        let fit_params = self.fit_params.as_ref().unwrap().to_vec(n, false);
        let fit_params_ep = [1.0; PARAMS_N];
        let fit_params_em = [1.0; PARAMS_N];
        // Prepare storage for the profiles and the reduced parallaxes
        let triple = vec![Triple::<F>::default(); 4];
        let triples = Rc::new(RefCell::new(vec![triple; self.objects.borrow().len()]));

        // Prepare a copy of the objects, so there are not affected by
        // the newly blacklisted ones in other points
        let objects = Rc::new(RefCell::new(self.objects.borrow().clone()));

        // Compute frozen profiles (all parameters are
        // fixed, but one is externally varied)
        //
        // We compute for the first parameter
        // only here (R_0) for debug purposes
        let len = fit_params.len();
        for index in 0..len {
            // Don't compute for the sigmas or compute for the sigmas only
            if Params::<F>::compute_with_l_stroke(index, l_stroke) {
                continue;
            }

            let fit_param = fit_params[index];
            let fit_param_ep = fit_params_ep[index];
            let fit_param_em = fit_params_em[index];

            let start = fit_param - fit_param_em;
            let end = fit_param + fit_param_ep;
            let h = (end - start) / F::from(POINTS_N).unwrap();

            let mut profile = Vec::<ProfilePoint<F>>::with_capacity(POINTS_N);

            let mut p = fit_params.clone();

            for j in 0..=POINTS_N {
                let param = start + F::from(j).unwrap() * h;

                let problem = OuterOptimizationProblem {
                    disable_inner: self.disable_inner,
                    objects: &objects,
                    params: &self.params,
                    triples: &Rc::clone(&triples),
                    output_dir: &self.output_dir,
                };

                p[index] = param;
                let cost = problem.inner_cost(&p, true)?;

                profile.push(ProfilePoint { param, cost });
            }

            self.serialize_to_profile(&ProfileType::Frozen, &profile, PARAMS_NAMES[index])
                .with_context(|| "Couldn't write a frozen profile to a file")?;
        }

        Ok(())
    }
    /// Serialize the profiles
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn serialize_to_profile(
        &self,
        profile_type: &ProfileType,
        profile: &Profile<F>,
        param_name: &str,
    ) -> Result<()>
    where
        F: Float + Debug + Display + Serialize,
    {
        let description_prefix = match *profile_type {
            ProfileType::Conditional => "Conditional",
            ProfileType::Frozen => "Frozen",
        };
        let file_prefix = match *profile_type {
            ProfileType::Conditional => "conditional",
            ProfileType::Frozen => "frozen",
        };

        // Prepare a header
        let params = &self.params;
        let fit_params = self.fit_params.as_ref().unwrap();
        let records = &profile;

        let header = formatdoc!(
            "
            # {description_prefix} profile of {param_name}
            {sample_description}
            # Descriptions:
            #
            # 01 param: Value of the parameter
            # 02 cost: Value of the cost function
            #
            # Parameters used:
            #
            # Galactocentric distance to the Sun [kpc]
            # R_0: {r_0}
            #
            # Circular velocity of the Sun at R = R_0 [km/s/kpc]
            # OMEGA_0: {omega_0}
            #
            # Oort's A constant [km/s/kpc]
            # A: {a}
            #
            # Residual motion of the Sun toward GC [km/s]
            # U_SUN: {u_sun}
            #
            # Residual motion of the Sun toward l = 90 degrees [km/s]
            # V_SUN: {v_sun}
            #
            # Residual motion of the Sun toward NGP [km/s]
            # W_SUN: {w_sun}
            #
            # Radial component of the ellipsoid of natural standard deviations [km/s]
            # SIGMA_R: {sigma_r_g}
            #
            # Azimuthal component of the ellipsoid of natural standard deviations [km/s]
            # SIGMA_THETA: {sigma_theta}
            #
            # Vertical component of the ellipsoid of natural standard deviations [km/s]
            # SIGMA_Z: {sigma_z}
            #
            # Constant parameters used:
            #
            # The right ascension of the north galactic pole [radians]
            # ALPHA_NGP: {alpha_ngp}
            #
            # The declination of the north galactic pole [radians]
            # DELTA_NGP: {delta_ngp}
            #
            # The longitude of the north celestial pole [radians]
            # L_NCP: {l_ncp}
            #
            # Linear velocities units conversion coefficient
            # K: {k}
            #
            # Standard Solar Motion toward GC [km/s]
            # U_SUN_STANDARD: {u_sun_standard}
            #
            # Standard Solar Motion toward l = 90 degrees [km/s]
            # V_SUN_STANDARD: {v_sun_standard}
            #
            # Standard Solar Motion toward NGP [km/s]
            # W_SUN_STANDARD: {w_sun_standard}
            #
            ",
            sample_description = self.format_sample_description(),
            r_0 = fit_params.r_0,
            omega_0 = fit_params.omega_0,
            a = fit_params.a,
            u_sun = fit_params.u_sun,
            v_sun = fit_params.v_sun,
            w_sun = fit_params.w_sun,
            sigma_r_g = fit_params.sigma_r_g,
            sigma_theta = fit_params.sigma_theta,
            sigma_z = fit_params.sigma_z,
            alpha_ngp = params.alpha_ngp,
            delta_ngp = params.delta_ngp,
            l_ncp = params.l_ncp,
            k = params.k,
            u_sun_standard = params.u_sun_standard,
            v_sun_standard = params.v_sun_standard,
            w_sun_standard = params.w_sun_standard,
        );

        let mut file_name = file_prefix.to_owned();
        file_name.push_str("_profile_");
        file_name.push_str(param_name);
        output::serialize_to(&self.output_dir, &file_name, &header, records)?;
        Ok(())
    }
}
