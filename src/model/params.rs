//! Model parameters

use super::io::output;
use super::Model;

use core::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;

use anyhow::{Context, Result};
use indoc::{formatdoc, indoc};
use num::Float;
use serde::Serialize;

/// Number of the optimized parameters
pub const PARAMS_N: usize = 18;

/// Maximum degree of the polynomial
/// of the rotation curve supported
pub const N_MAX: usize = 10;

/// Names of the optimized parameters
pub const PARAMS_NAMES: [&str; PARAMS_N] = [
    "R_0",
    "omega_0",
    "A",
    "U_sun",
    "V_sun",
    "W_sun",
    "sigma_R",
    "sigma_theta",
    "sigma_Z",
    "theta_2",
    "theta_3",
    "theta_4",
    "theta_5",
    "theta_6",
    "theta_7",
    "theta_8",
    "theta_9",
    "theta_10",
];

/// Model parameters
#[derive(Default, Debug, Clone, Serialize)]
pub struct Params<F> {
    /// Galactocentric distance to the Sun (kpc)
    #[serde(rename = "R_0")]
    pub r_0: F,
    /// Plus uncertainty in `r_0`
    #[serde(rename = "R_0_ep")]
    pub r_0_ep: F,
    /// Minus uncertainty in `r_0`
    #[serde(rename = "R_0_em")]
    pub r_0_em: F,
    /// Circular velocity of the Sun at R = R_0 (km/s/kpc)
    pub omega_0: F,
    /// Plus uncertainty in `omega_0`
    pub omega_0_ep: F,
    /// Minus uncertainty in `omega_0`
    pub omega_0_em: F,
    /// Oort's A constant (km/s/kpc)
    #[serde(rename = "A")]
    pub a: F,
    /// Plus uncertainty in `a`
    #[serde(rename = "A_ep")]
    pub a_ep: F,
    /// Minus uncertainty in `a`
    #[serde(rename = "A_em")]
    pub a_em: F,
    /// Residual motion of the Sun toward GC (km/s)
    #[serde(rename = "U_sun")]
    pub u_sun: F,
    /// Plus uncertainty in `u_sun`
    #[serde(rename = "U_sun_ep")]
    pub u_sun_ep: F,
    /// Minus uncertainty in `u_sun`
    #[serde(rename = "U_sun_em")]
    pub u_sun_em: F,
    /// Residual motion of the Sun toward l = 90 degrees (km/s)
    #[serde(rename = "V_sun")]
    pub v_sun: F,
    /// Plus uncertainty in `v_sun`
    #[serde(rename = "V_sun_ep")]
    pub v_sun_ep: F,
    /// Minus uncertainty in `v_sun`
    #[serde(rename = "V_sun_em")]
    pub v_sun_em: F,
    /// Residual motion of the Sun toward NGP (km/s)
    #[serde(rename = "W_sun")]
    pub w_sun: F,
    /// Plus uncertainty in `w_sun`
    #[serde(rename = "W_sun_ep")]
    pub w_sun_ep: F,
    /// Minus uncertainty in `w_sun`
    #[serde(rename = "W_sun_em")]
    pub w_sun_em: F,
    /// Radial component of the ellipsoid of natural standard deviations (km/s)
    #[serde(rename = "sigma_R")]
    pub sigma_r_g: F,
    /// Plus uncertainty in `sigma_r`
    #[serde(rename = "sigma_R_ep")]
    pub sigma_r_g_ep: F,
    /// Minus uncertainty in `sigma_r`
    #[serde(rename = "sigma_R_em")]
    pub sigma_r_g_em: F,
    /// Azimuthal component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_theta: F,
    /// Plus uncertainty in `sigma_theta`
    pub sigma_theta_ep: F,
    /// Minus uncertainty in `sigma_theta`
    pub sigma_theta_em: F,
    /// Vertical component of the ellipsoid of natural standard deviations (km/s)
    #[serde(rename = "sigma_Z")]
    pub sigma_z: F,
    /// Plus uncertainty in `sigma_z`
    #[serde(rename = "sigma_Z_ep")]
    pub sigma_z_ep: F,
    /// Minus uncertainty in `sigma_z`
    #[serde(rename = "sigma_Z_em")]
    pub sigma_z_em: F,
    /// The second derivative of the linear rotation velocity (km/s/kpc^2)
    pub theta_2: F,
    /// Plus uncertainty in `theta_2`
    pub theta_2_ep: F,
    /// Minus uncertainty in `theta_2`
    pub theta_2_em: F,
    /// The third derivative of the linear rotation velocity (km/s/kpc^3)
    pub theta_3: F,
    /// Plus uncertainty in `theta_3`
    pub theta_3_ep: F,
    /// Minus uncertainty in `theta_3`
    pub theta_3_em: F,
    /// The 4th derivative of the linear rotation velocity (km/s/kpc^4)
    pub theta_4: F,
    /// Plus uncertainty in `theta_4`
    pub theta_4_ep: F,
    /// Minus uncertainty in `theta_4`
    pub theta_4_em: F,
    /// The 5th derivative of the linear rotation velocity (km/s/kpc^5)
    pub theta_5: F,
    /// Plus uncertainty in `theta_5`
    pub theta_5_ep: F,
    /// Minus uncertainty in `theta_5`
    pub theta_5_em: F,
    /// The 6th derivative of the linear rotation velocity (km/s/kpc^6)
    pub theta_6: F,
    /// Plus uncertainty in `theta_6`
    pub theta_6_ep: F,
    /// Minus uncertainty in `theta_6`
    pub theta_6_em: F,
    /// The 7th derivative of the linear rotation velocity (km/s/kpc^7)
    pub theta_7: F,
    /// Plus uncertainty in `theta_7`
    pub theta_7_ep: F,
    /// Minus uncertainty in `theta_7`
    pub theta_7_em: F,
    /// The 8th derivative of the linear rotation velocity (km/s/kpc^8)
    pub theta_8: F,
    /// Plus uncertainty in `theta_8`
    pub theta_8_ep: F,
    /// Minus uncertainty in `theta_8`
    pub theta_8_em: F,
    /// The 9th derivative of the linear rotation velocity (km/s/kpc^9)
    pub theta_9: F,
    /// Plus uncertainty in `theta_9`
    pub theta_9_ep: F,
    /// Minus uncertainty in `theta_9`
    pub theta_9_em: F,
    /// The 10th derivative of the linear rotation velocity (km/s/kpc^10)
    pub theta_10: F,
    /// Plus uncertainty in `theta_10`
    pub theta_10_ep: F,
    /// Minus uncertainty in `theta_10`
    pub theta_10_em: F,
    /// The constant term of the rotation curve (km/s)
    pub theta_0: F,
    /// The first derivative of the linear rotation velocity (km/s/kpc)
    pub theta_1: F,
    /// Linear rotation velocity of the Sun (km/s)
    pub theta_sun: F,
    /// The right ascension of the north galactic pole (radians)
    #[serde(skip)]
    pub alpha_ngp: F,
    /// The declination of the north galactic pole (radians)
    #[serde(skip)]
    pub delta_ngp: F,
    /// The longitude of the north celestial pole (radians)
    #[serde(skip)]
    pub l_ncp: F,
    /// Linear velocities units conversion coefficient
    #[serde(skip)]
    pub k: F,
    /// Standard Solar Motion toward GC (km/s)
    #[serde(skip)]
    pub u_sun_standard: F,
    /// Standard Solar Motion toward l = 90 degrees (km/s)
    #[serde(skip)]
    pub v_sun_standard: F,
    /// Standard Solar Motion toward NGP (km/s)
    #[serde(skip)]
    pub w_sun_standard: F,
}

impl<F> Params<F> {
    /// Update the parameters with the point in the parameter space
    ///
    /// Note that not all fields are updated, but only those needed for fitting
    #[allow(clippy::indexing_slicing)]
    pub fn update_with(&mut self, p: &[F])
    where
        F: Float + Debug,
    {
        let len = p.len();
        let mut new_p = [F::zero(); PARAMS_N];
        new_p[0..len].copy_from_slice(&p[0..len]);

        self.r_0 = new_p[0];
        self.omega_0 = new_p[1];
        self.a = new_p[2];
        self.u_sun = new_p[3];
        self.v_sun = new_p[4];
        self.w_sun = new_p[5];
        self.sigma_r_g = new_p[6];
        self.sigma_theta = new_p[7];
        self.sigma_z = new_p[8];
        self.theta_2 = new_p[9];
        self.theta_3 = new_p[10];
        self.theta_4 = new_p[11];
        self.theta_5 = new_p[12];
        self.theta_6 = new_p[13];
        self.theta_7 = new_p[14];
        self.theta_8 = new_p[15];
        self.theta_9 = new_p[16];
        self.theta_10 = new_p[17];
    }
    /// Construct a point in the parameter space from the parameters
    ///
    /// Note that not all fields are used, but only those needed for fitting
    #[allow(clippy::indexing_slicing)]
    pub fn to_vec(&self, n: usize) -> Vec<F>
    where
        F: Float + Debug,
    {
        let array = [
            self.r_0,
            self.omega_0,
            self.a,
            self.u_sun,
            self.v_sun,
            self.w_sun,
            self.sigma_r_g,
            self.sigma_theta,
            self.sigma_z,
            self.theta_2,
            self.theta_3,
            self.theta_4,
            self.theta_5,
            self.theta_6,
            self.theta_7,
            self.theta_8,
            self.theta_9,
            self.theta_10,
        ];
        let slice = &array[0..=8 + (n - 1)];
        slice.to_vec()
    }
    /// Update the plus uncertainties of the parameters
    /// with the values in the provided vector
    ///
    /// Note that not all fields are updated, but only those needed for fitting
    #[allow(clippy::indexing_slicing)]
    pub fn update_ep_with(&mut self, p: &[F])
    where
        F: Float + Debug,
    {
        let len = p.len();
        let mut new_p = [F::zero(); PARAMS_N];
        new_p[0..len].copy_from_slice(&p[0..len]);

        self.r_0_ep = new_p[0];
        self.omega_0_ep = new_p[1];
        self.a_ep = new_p[2];
        self.u_sun_ep = new_p[3];
        self.v_sun_ep = new_p[4];
        self.w_sun_ep = new_p[5];
        self.sigma_r_g_ep = new_p[6];
        self.sigma_theta_ep = new_p[7];
        self.sigma_z_ep = new_p[8];
        self.theta_2_ep = new_p[9];
        self.theta_3_ep = new_p[10];
        self.theta_4_ep = new_p[11];
        self.theta_5_ep = new_p[12];
        self.theta_6_ep = new_p[13];
        self.theta_7_ep = new_p[14];
        self.theta_8_ep = new_p[15];
        self.theta_9_ep = new_p[16];
        self.theta_10_ep = new_p[17];
    }
    /// Put the plus uncertainties into a vector
    #[allow(clippy::indexing_slicing)]
    pub fn to_ep_vec(&self, n: usize) -> Vec<F>
    where
        F: Float + Debug,
    {
        let array = [
            self.r_0_ep,
            self.omega_0_ep,
            self.a_ep,
            self.u_sun_ep,
            self.v_sun_ep,
            self.w_sun_ep,
            self.sigma_r_g_ep,
            self.sigma_theta_ep,
            self.sigma_z_ep,
            self.theta_2_ep,
            self.theta_3_ep,
            self.theta_4_ep,
            self.theta_5_ep,
            self.theta_6_ep,
            self.theta_7_ep,
            self.theta_8_ep,
            self.theta_9_ep,
            self.theta_10_ep,
        ];
        let slice = &array[0..=8 + (n - 1)];
        slice.to_vec()
    }
    /// Update the minus uncertainties of the parameters
    /// with the values in the provided vector
    ///
    /// Note that not all fields are updated, but only those needed for fitting
    #[allow(clippy::indexing_slicing)]
    pub fn update_em_with(&mut self, p: &[F])
    where
        F: Float + Debug,
    {
        let len = p.len();
        let mut new_p = [F::zero(); PARAMS_N];
        new_p[0..len].copy_from_slice(&p[0..len]);

        self.r_0_em = new_p[0];
        self.omega_0_em = new_p[1];
        self.a_em = new_p[2];
        self.u_sun_em = new_p[3];
        self.v_sun_em = new_p[4];
        self.w_sun_em = new_p[5];
        self.sigma_r_g_em = new_p[6];
        self.sigma_theta_em = new_p[7];
        self.sigma_z_em = new_p[8];
        self.theta_2_em = new_p[9];
        self.theta_3_em = new_p[10];
        self.theta_4_em = new_p[11];
        self.theta_5_em = new_p[12];
        self.theta_6_em = new_p[13];
        self.theta_7_em = new_p[14];
        self.theta_8_em = new_p[15];
        self.theta_9_em = new_p[16];
        self.theta_10_em = new_p[17];
    }
    /// Put the minus uncertainties into a vector
    #[allow(clippy::indexing_slicing)]
    pub fn to_em_vec(&self, n: usize) -> Vec<F>
    where
        F: Float + Debug,
    {
        let array = [
            self.r_0_em,
            self.omega_0_em,
            self.a_em,
            self.u_sun_em,
            self.v_sun_em,
            self.w_sun_em,
            self.sigma_r_g_em,
            self.sigma_theta_em,
            self.sigma_z_em,
            self.theta_2_em,
            self.theta_3_em,
            self.theta_4_em,
            self.theta_5_em,
            self.theta_6_em,
            self.theta_7_em,
            self.theta_8_em,
            self.theta_9_em,
            self.theta_10_em,
        ];
        let slice = &array[0..=8 + (n - 1)];
        slice.to_vec()
    }
}

/// Descriptions of the fields
const DESCRIPTIONS: &str = indoc!(
    "
    # 01 R_0: Galactocentric distance to the Sun [kpc]
    # 02 R_0_ep: Plus uncertainty in `R_0` [kpc]
    # 03 R_0_em: Minus uncertainty in `R_0` [kpc]
    # 04 omega_0: Circular velocity of the Sun at R = R_0 [km/s/kpc]
    # 05 omega_0_ep: Plus uncertainty in `omega_0` [km/s/kpc]
    # 06 omega_0_em: Minus uncertainty in `omega_0` [km/s/kpc]
    # 07 A: Oort's A constant [km/s/kpc]
    # 08 A_ep: Plus uncertainty in `A` [km/s/kpc]
    # 09 A_em: Minus uncertainty in `A` [km/s/kpc]
    # 10 U_sun: Residual motion of the Sun toward GC [km/s]
    # 11 U_ep: Plus uncertainty in `U` [km/s]
    # 12 U_em: Minus uncertainty in `U` [km/s]
    # 13 V_sun: Residual motion of the Sun toward l = 90 degrees [km/s]
    # 14 V_ep: Plus uncertainty in `V` [km/s]
    # 15 V_em: Minus uncertainty in `V` [km/s]
    # 16 W_sun: Residual motion of the Sun toward NGP [km/s]
    # 17 W_ep: Plus uncertainty in `W` [km/s]
    # 18 W_em: Minus uncertainty in `W` [km/s]
    # 19 sigma_R: Radial component of the ellipsoid of natural standard deviations [km/s]
    # 20 sigma_R_ep: Plus uncertainty in `sigma_R` [km/s]
    # 21 sigma_R_em: Minus uncertainty in `sigma_R` [km/s]
    # 22 sigma_theta: Azimuthal component of the ellipsoid of natural standard deviations [km/s]
    # 23 sigma_theta_ep: Plus uncertainty in `sigma_theta` [km/s]
    # 24 sigma_theta_em: Minus uncertainty in `sigma_theta` [km/s]
    # 25 sigma_Z: Vertical component of the ellipsoid of natural standard deviations [km/s]
    # 26 sigma_Z_ep: Plus uncertainty in `sigma_Z` [km/s]
    # 27 sigma_Z_em: Minus uncertainty in `sigma_Z` [km/s]
    # 28 theta_2: The second derivative of the linear rotation velocity [km/s/kpc^2]
    # 29 theta_2_ep: Plus uncertainty in `theta_2` [km/s/kpc^2]
    # 30 theta_2_em: Minus uncertainty in `theta_2` [km/s/kpc^2]
    # 31 theta_3: The third derivative of the linear rotation velocity [km/s/kpc^3]
    # 32 theta_3_ep: Plus uncertainty in `theta_3` [km/s/kpc^3]
    # 33 theta_3_em: Minus uncertainty in `theta_3` [km/s/kpc^3]
    # 34 theta_4: The 4th derivative of the linear rotation velocity [km/s/kpc^4]
    # 35 theta_4_ep: Plus uncertainty in `theta_4` [km/s/kpc^4]
    # 36 theta_4_em: Minus uncertainty in `theta_4` [km/s/kpc^4]
    # 37 theta_5: The 5th derivative of the linear rotation velocity [km/s/kpc^5]
    # 38 theta_5_ep: Plus uncertainty in `theta_5` [km/s/kpc^5]
    # 39 theta_5_em: Minus uncertainty in `theta_5` [km/s/kpc^5]
    # 40 theta_6: The 6th derivative of the linear rotation velocity [km/s/kpc^6]
    # 41 theta_6_ep: Plus uncertainty in `theta_6` [km/s/kpc^6]
    # 42 theta_6_em: Minus uncertainty in `theta_6` [km/s/kpc^6]
    # 43 theta_7: The 7th derivative of the linear rotation velocity [km/s/kpc^7]
    # 44 theta_7_ep: Plus uncertainty in `theta_7` [km/s/kpc^7]
    # 45 theta_7_em: Minus uncertainty in `theta_7` [km/s/kpc^7]
    # 46 theta_8: The 8th derivative of the linear rotation velocity [km/s/kpc^8]
    # 47 theta_8_ep: Plus uncertainty in `theta_8` [km/s/kpc^8]
    # 48 theta_8_em: Minus uncertainty in `theta_8` [km/s/kpc^8]
    # 49 theta_9: The 9th derivative of the linear rotation velocity [km/s/kpc^9]
    # 50 theta_9_ep: Plus uncertainty in `theta_9` [km/s/kpc^9]
    # 51 theta_9_em: Minus uncertainty in `theta_9` [km/s/kpc^9]
    # 52 theta_10: The 10th derivative of the linear rotation velocity [km/s/kpc^10]
    # 53 theta_10_ep: Plus uncertainty in `theta_10` [km/s/kpc^10]
    # 54 theta_10_em: Minus uncertainty in `theta_10` [km/s/kpc^10]
    # 55 theta_0: The constant term of the rotation curve [km/s]
    # 56 theta_1: The first derivative of the linear rotation velocity [km/s/kpc]
    # 57 theta_sun: Linear rotation velocity of the Sun [km/s]"
);

impl<F> Model<F> {
    /// Serialize the initial parameters
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub(in crate::model) fn serialize_to_params(&self) -> Result<()>
    where
        F: Float + Debug + Display + Serialize,
    {
        // Prepare a header
        let params = &self.params;
        let header = formatdoc!(
            "
            # Initial parameters
            {sample_description}
            # Descriptions:
            #
            {DESCRIPTIONS}
            #
            # Note that the last 3 values are derived from the other parameters.
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
            alpha_ngp = params.alpha_ngp,
            delta_ngp = params.delta_ngp,
            l_ncp = params.l_ncp,
            k = params.k,
            u_sun_standard = params.u_sun_standard,
            v_sun_standard = params.v_sun_standard,
            w_sun_standard = params.w_sun_standard,
        );
        let records = vec![params];
        output::serialize_to(&self.output_dir, "params", &header, &records)
    }
    /// Serialize the fitted parameters
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub fn serialize_to_fit_params(&self) -> Result<()>
    where
        F: Float + Debug + Display + Serialize,
    {
        // Prepare a header
        let params = &self.params;
        let fit_params = self.fit_params.as_ref().unwrap();
        let header = formatdoc!(
            "
            # Fit of the model (parameters)
            {sample_description}
            # Descriptions:
            #
            {DESCRIPTIONS}
            #
            # Note that the last 3 values are derived from the other parameters.
            #
            # Initial parameters used:
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
            r_0 = params.r_0,
            omega_0 = params.omega_0,
            a = params.a,
            u_sun = params.u_sun,
            v_sun = params.v_sun,
            w_sun = params.w_sun,
            sigma_r_g = params.sigma_r_g,
            sigma_theta = params.sigma_theta,
            sigma_z = params.sigma_z,
            alpha_ngp = params.alpha_ngp,
            delta_ngp = params.delta_ngp,
            l_ncp = params.l_ncp,
            k = params.k,
            u_sun_standard = params.u_sun_standard,
            v_sun_standard = params.v_sun_standard,
            w_sun_standard = params.w_sun_standard,
        );
        let name = "fit_params";
        let records = vec![fit_params];
        output::serialize_to(&self.output_dir, name, &header, &records)?;
        // Represent in a plain view, too
        let plain_path = &self.output_dir.join(format!("{name}.plain"));
        let mut plain_file = File::create(plain_path)
            .with_context(|| format!("Couldn't open the file {plain_path:?} in write-only mode"))?;
        write!(
            &mut plain_file,
            "{}",
            formatdoc!("
            Fit of the model (parameters)
            {sample_description}
            Optimization results:

                      n: {n}
                    L_1: {best_cost}

                      R: {r_0:>20.15} -> {fit_r_0:>20.15} + {fit_r_0_ep:>17.15} - {fit_r_0_em:>17.15}
                omega_0: {omega_0:>20.15} -> {fit_omega_0:>20.15} + {fit_omega_0_ep:>17.15} - {fit_omega_0_em:>17.15}
                      A: {a:>20.15} -> {fit_a:>20.15} + {fit_a_ep:>17.15} - {fit_a_em:>17.15}
                  U_sun: {u_sun:>20.15} -> {fit_u_sun:>20.15} + {fit_u_sun_ep:>17.15} - {fit_u_sun_em:>17.15}
                  V_sun: {v_sun:>20.15} -> {fit_v_sun:>20.15} + {fit_v_sun_ep:>17.15} - {fit_v_sun_em:>17.15}
                  W_sun: {w_sun:>20.15} -> {fit_w_sun:>20.15} + {fit_w_sun_ep:>17.15} - {fit_w_sun_em:>17.15}
                sigma_R: {sigma_r_g:>20.15} -> {fit_sigma_r_g:>20.15} + {fit_sigma_r_g_ep:>17.15} - {fit_sigma_r_g_em:>17.15}
            sigma_theta: {sigma_theta:>20.15} -> {fit_sigma_theta:>20.15} + {fit_sigma_theta_ep:>17.15} - {fit_sigma_theta_em:>17.15}
                sigma_Z: {sigma_z:>20.15} -> {fit_sigma_z:>20.15} + {fit_sigma_z_ep:>17.15} - {fit_sigma_z_em:>17.15}
                theta_2: {theta_2:>20.15} -> {fit_theta_2:>20.15} + {fit_theta_2_ep:>17.15} - {fit_theta_2_em:>17.15}
                theta_3: {theta_3:>20.15} -> {fit_theta_3:>20.15} + {fit_theta_3_ep:>17.15} - {fit_theta_3_em:>17.15}
                theta_4: {theta_4:>20.15} -> {fit_theta_4:>20.15} + {fit_theta_4_ep:>17.15} - {fit_theta_4_em:>17.15}
                theta_5: {theta_5:>20.15} -> {fit_theta_5:>20.15} + {fit_theta_5_ep:>17.15} - {fit_theta_5_em:>17.15}
                theta_6: {theta_6:>20.15} -> {fit_theta_6:>20.15} + {fit_theta_6_ep:>17.15} - {fit_theta_6_em:>17.15}
                theta_7: {theta_7:>20.15} -> {fit_theta_7:>20.15} + {fit_theta_7_ep:>17.15} - {fit_theta_7_em:>17.15}
                theta_8: {theta_8:>20.15} -> {fit_theta_8:>20.15} + {fit_theta_8_ep:>17.15} - {fit_theta_8_em:>17.15}
                theta_9: {theta_9:>20.15} -> {fit_theta_9:>20.15} + {fit_theta_9_ep:>17.15} - {fit_theta_9_em:>17.15}
               theta_10: {theta_10:>20.15} -> {fit_theta_10:>20.15} + {fit_theta_10_ep:>17.15} - {fit_theta_10_em:>17.15}

            Derived values:

                theta_0: {theta_0:>19.15} -> {fit_theta_0:>19.15}
                theta_1: {theta_1:>19.15} -> {fit_theta_1:>19.15}
              theta_sun: {theta_sun:>19.15} -> {fit_theta_sun:>19.15}

            Constant parameters used:

            The right ascension of the north galactic pole [radians]
            ALPHA_NGP: {alpha_ngp}

            The declination of the north galactic pole [radians]
            DELTA_NGP: {delta_ngp}

            The longitude of the north celestial pole [radians]
            L_NCP: {l_ncp}

            Linear velocities units conversion coefficient
            K: {k}

            Standard Solar Motion toward GC [km/s]
            U_SUN_STANDARD: {u_sun_standard}

            Standard Solar Motion toward l = 90 degrees [km/s]
            V_SUN_STANDARD: {v_sun_standard}

            Standard Solar Motion toward NGP [km/s]
            W_SUN_STANDARD: {w_sun_standard}
            ",
                sample_description = self.format_sample_description().replace("# ", "").replace('#', ""),
                n = self.n.unwrap(),
                best_cost = self.best_cost.unwrap(),
                r_0 = params.r_0,
                omega_0 = params.omega_0,
                a = params.a,
                u_sun = params.u_sun,
                v_sun = params.v_sun,
                w_sun = params.w_sun,
                sigma_r_g = params.sigma_r_g,
                sigma_theta = params.sigma_theta,
                sigma_z = params.sigma_z,
                theta_2 = params.theta_2,
                theta_3 = params.theta_3,
                theta_4 = params.theta_4,
                theta_5 = params.theta_5,
                theta_6 = params.theta_6,
                theta_7 = params.theta_7,
                theta_8 = params.theta_8,
                theta_9 = params.theta_9,
                theta_10 = params.theta_10,
                theta_0 = params.theta_0,
                theta_1 = params.theta_1,
                theta_sun = params.theta_sun,
                fit_r_0 = fit_params.r_0,
                fit_omega_0 = fit_params.omega_0,
                fit_a = fit_params.a,
                fit_u_sun = fit_params.u_sun,
                fit_v_sun = fit_params.v_sun,
                fit_w_sun = fit_params.w_sun,
                fit_sigma_r_g = fit_params.sigma_r_g,
                fit_sigma_theta = fit_params.sigma_theta,
                fit_sigma_z = fit_params.sigma_z,
                fit_r_0_ep = fit_params.r_0_ep,
                fit_omega_0_ep = fit_params.omega_0_ep,
                fit_a_ep = fit_params.a_ep,
                fit_u_sun_ep = fit_params.u_sun_ep,
                fit_v_sun_ep = fit_params.v_sun_ep,
                fit_w_sun_ep = fit_params.w_sun_ep,
                fit_sigma_r_g_ep = fit_params.sigma_r_g_ep,
                fit_sigma_theta_ep = fit_params.sigma_theta_ep,
                fit_sigma_z_ep = fit_params.sigma_z_ep,
                fit_r_0_em = fit_params.r_0_em,
                fit_omega_0_em = fit_params.omega_0_em,
                fit_a_em = fit_params.a_em,
                fit_u_sun_em = fit_params.u_sun_em,
                fit_v_sun_em = fit_params.v_sun_em,
                fit_w_sun_em = fit_params.w_sun_em,
                fit_sigma_r_g_em = fit_params.sigma_r_g_em,
                fit_sigma_theta_em = fit_params.sigma_theta_em,
                fit_sigma_z_em = fit_params.sigma_z_em,
                fit_theta_2 = fit_params.theta_2,
                fit_theta_3 = fit_params.theta_3,
                fit_theta_4 = fit_params.theta_4,
                fit_theta_5 = fit_params.theta_5,
                fit_theta_6 = fit_params.theta_6,
                fit_theta_7 = fit_params.theta_7,
                fit_theta_8 = fit_params.theta_8,
                fit_theta_9 = fit_params.theta_9,
                fit_theta_10 = fit_params.theta_10,
                fit_theta_2_ep = fit_params.theta_2_ep,
                fit_theta_3_ep = fit_params.theta_3_ep,
                fit_theta_4_ep = fit_params.theta_4_ep,
                fit_theta_5_ep = fit_params.theta_5_ep,
                fit_theta_6_ep = fit_params.theta_6_ep,
                fit_theta_7_ep = fit_params.theta_7_ep,
                fit_theta_8_ep = fit_params.theta_8_ep,
                fit_theta_9_ep = fit_params.theta_9_ep,
                fit_theta_10_ep = fit_params.theta_10_ep,
                fit_theta_2_em = fit_params.theta_2_em,
                fit_theta_3_em = fit_params.theta_3_em,
                fit_theta_4_em = fit_params.theta_4_em,
                fit_theta_5_em = fit_params.theta_5_em,
                fit_theta_6_em = fit_params.theta_6_em,
                fit_theta_7_em = fit_params.theta_7_em,
                fit_theta_8_em = fit_params.theta_8_em,
                fit_theta_9_em = fit_params.theta_9_em,
                fit_theta_10_em = fit_params.theta_10_em,
                fit_theta_0 = fit_params.theta_0,
                fit_theta_1 = fit_params.theta_1,
                fit_theta_sun = fit_params.theta_sun,
                alpha_ngp = params.alpha_ngp,
                delta_ngp = params.delta_ngp,
                l_ncp = params.l_ncp,
                k = params.k,
                u_sun_standard = params.u_sun_standard,
                v_sun_standard = params.v_sun_standard,
                w_sun_standard = params.w_sun_standard,
            )
        )
        .with_context(|| format!("Couldn't write to {plain_path:?}"))?;
        Ok(())
    }
}
