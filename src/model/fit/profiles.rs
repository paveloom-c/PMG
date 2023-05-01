//! Profiles

extern crate alloc;

use super::io::output;
use super::params::{ARMIJO_PARAM, BACKTRACKING_PARAM, LBFGS_M, LBFGS_TOLERANCE};
use super::{FrozenOuterOptimizationProblem, OuterOptimizationProblem, Triple};
use super::{Model, PARAMS_N, PARAMS_NAMES};
use crate::utils::FiniteDiff;

use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use core::iter::Sum;

use anyhow::{Context, Result};
use argmin::core::{ArgminFloat, Executor};
use argmin::solver::linesearch::condition::ArmijoCondition;
use argmin::solver::linesearch::BacktrackingLineSearch;
use argmin::solver::quasinewton::LBFGS;
use argmin_math::{
    ArgminAdd, ArgminDot, ArgminL1Norm, ArgminL2Norm, ArgminMinMax, ArgminMul, ArgminSignum,
    ArgminSub, ArgminZeroLike,
};
use indoc::formatdoc;
use itertools::izip;
use num::Float;
use numeric_literals::replace_float_literals;
use serde::Serialize;

/// Profiles
pub type Profiles<F> = Vec<Profile<F>>;

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
    #[allow(clippy::similar_names)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn try_compute_conditional_profiles(&mut self, n: usize) -> Result<()>
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
        let fit_params = self.fit_params.as_ref().unwrap().to_vec(n);
        let fit_params_ep = [1.0; PARAMS_N];
        let fit_params_em = [1.0; PARAMS_N];
        // Prepare storage
        let mut profiles = Profiles::<F>::with_capacity(PARAMS_N);
        let triple = vec![Triple::<F>::default(); 4];
        let triples = Rc::new(RefCell::new(vec![triple; self.objects.borrow().len()]));

        // Compute conditional profiles (one parameter is fixed
        // and externally varied, the rest are free)
        for index in 0..=(8 + (n - 1)) {
            let fit_param = fit_params[index];
            let fit_param_ep = fit_params_ep[index];
            let fit_param_em = fit_params_em[index];

            let coeff = 1.1;
            let start = fit_param - fit_param_em * coeff;
            let end = fit_param + fit_param_ep * coeff;
            let h = (end - start) / F::from(POINTS_N).unwrap();

            let mut profile = Vec::<ProfilePoint<F>>::with_capacity(POINTS_N);

            for j in 0..=POINTS_N {
                let param = start + F::from(j).unwrap() * h;

                let problem = FrozenOuterOptimizationProblem {
                    index,
                    param,
                    objects: &self.objects,
                    params: &self.params,
                    triples: &Rc::clone(&triples),
                };
                let mut init_param = self.params.to_vec(n);
                // Remove the frozen parameter
                init_param.remove(index);
                let cond = ArmijoCondition::new(F::from(ARMIJO_PARAM).unwrap())?;
                let linesearch =
                    BacktrackingLineSearch::new(cond).rho(F::from(BACKTRACKING_PARAM).unwrap())?;
                let solver = LBFGS::new(linesearch, LBFGS_M)
                    .with_tolerance_cost(F::from(LBFGS_TOLERANCE).unwrap())?;
                // Find the local minimum in the outer optimization
                let res = Executor::new(problem, solver)
                    .configure(|state| state.param(init_param))
                    .run()
                    .with_context(|| {
                        "Couldn't solve the outer optimization problem with a frozen parameter"
                    })?;
                let cost = res.state().get_best_cost();

                profile.push(ProfilePoint { param, cost });
            }

            self.serialize_to_profile(&ProfileType::Conditional, &profile, index)
                .with_context(|| "Couldn't write a conditional profile to a file")?;

            profiles.push(profile);
        }

        self.conditional_profiles = Some(profiles);

        Ok(())
    }
    /// Try to compute the frozen profiles
    #[allow(clippy::indexing_slicing)]
    #[allow(clippy::shadow_unrelated)]
    #[allow(clippy::similar_names)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    pub fn try_compute_frozen_profiles(&mut self, n: usize) -> Result<()>
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
        let fit_params = self.fit_params.as_ref().unwrap().to_vec(n);
        let fit_params_ep = [1.0; PARAMS_N];
        let fit_params_em = [1.0; PARAMS_N];
        // Prepare storage for the profiles and the reduced parallaxes
        let mut profiles = Profiles::<F>::with_capacity(PARAMS_N);
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
        for index in 0..=(8 + (n - 1)) {
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
                    objects: &objects,
                    params: &self.params,
                    triples: &Rc::clone(&triples),
                };

                p[index] = param;
                let cost = problem.inner_cost(&p, true, false)?;

                // Go through the final discrepancies and
                // decide whether some of them are too big
                for (object, triples) in izip!(
                    self.objects.borrow_mut().iter_mut(),
                    triples.borrow().iter()
                ) {
                    if !object.blacklisted {
                        let coeffs = [5., 5., 5.];
                        for (triple, coeff) in izip!(&triples[0..3], coeffs) {
                            let Triple {
                                ref observed,
                                ref model,
                                ref error,
                            } = *triple;
                            if (*observed - *model).abs() >= coeff * *error {
                                object.blacklisted = true;
                            }
                        }
                    }
                }

                profile.push(ProfilePoint { param, cost });
            }

            self.serialize_to_profile(&ProfileType::Frozen, &profile, index)
                .with_context(|| "Couldn't write a frozen profile to a file")?;

            profiles.push(profile);
        }

        self.frozen_profiles = Some(profiles);

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
        index: usize,
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
        let param_name = PARAMS_NAMES[index];

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
