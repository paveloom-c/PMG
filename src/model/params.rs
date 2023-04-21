//! Model parameters

use super::io::output;
use super::Model;

use core::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use indoc::{formatdoc, indoc};
use num::Float;
use serde::Serialize;

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
    /// The constant term of the rotation curve (km/s)
    pub theta_0: F,
    /// The first derivative of the rotation curve (km/s/kpc)
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
        self.r_0 = p[0];
        self.omega_0 = p[1];
        self.a = p[2];
        self.u_sun = p[3];
        self.v_sun = p[4];
        self.w_sun = p[5];
        self.sigma_r_g = p[6];
        self.sigma_theta = p[7];
        self.sigma_z = p[8];
    }
    /// Construct a point in the parameter space from the parameters
    ///
    /// Note that not all fields are used, but only those needed for fitting
    pub fn to_point(&self) -> Vec<F>
    where
        F: Float + Debug,
    {
        [
            self.r_0,
            self.omega_0,
            self.a,
            self.u_sun,
            self.v_sun,
            self.w_sun,
            self.sigma_r_g,
            self.sigma_theta,
            self.sigma_z,
        ]
        .to_vec()
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
        self.r_0_ep = p[0];
        self.omega_0_ep = p[1];
        self.a_ep = p[2];
        self.u_sun_ep = p[3];
        self.v_sun_ep = p[4];
        self.w_sun_ep = p[5];
        self.sigma_r_g_ep = p[6];
        self.sigma_theta_ep = p[7];
        self.sigma_z_ep = p[8];
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
        self.r_0_em = p[0];
        self.omega_0_em = p[1];
        self.a_em = p[2];
        self.u_sun_em = p[3];
        self.v_sun_em = p[4];
        self.w_sun_em = p[5];
        self.sigma_r_g_em = p[6];
        self.sigma_theta_em = p[7];
        self.sigma_z_em = p[8];
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
    # 28 theta_0: The constant term of the rotation curve [km/s]
    # 29 theta_1: The first derivative of the rotation curve [km/s/kpc]
    # 30 theta_sun: Linear rotation velocity of the Sun [km/s]"
);

impl<F> Model<F> {
    /// Serialize the initial parameters
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub(in crate::model) fn serialize_to_params(&self, output_dir: &Path) -> Result<()>
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
        output::serialize_to(output_dir, "params", &header, &records)
    }
    /// Serialize the fitted parameters
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub(in crate::model) fn serialize_to_fit_params(&self, output_dir: &Path) -> Result<()>
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
        output::serialize_to(output_dir, name, &header, &records)?;
        // Represent in a plain view, too
        let plain_path = &output_dir.join(format!("{name}.plain"));
        let mut plain_file = File::create(plain_path)
            .with_context(|| format!("Couldn't open the file {plain_path:?} in write-only mode"))?;
        write!(
            &mut plain_file,
            "{}",
            formatdoc!("
            Fit of the model (parameters)
            {sample_description}
            Parameters:

                      R: {r_0:>18.15} -> {fit_r_0:>18.15} + {fit_r_0_ep:>17.15} - {fit_r_0_em:>17.15}
                omega_0: {omega_0:>18.15} -> {fit_omega_0:>18.15} + {fit_omega_0_ep:>17.15} - {fit_omega_0_em:>17.15}
                      A: {a:>18.15} -> {fit_a:>18.15} + {fit_a_ep:>17.15} - {fit_a_em:>17.15}
                  U_sun: {u_sun:>18.15} -> {fit_u_sun:>18.15} + {fit_u_sun_ep:>17.15} - {fit_u_sun_em:>17.15}
                  V_sun: {v_sun:>18.15} -> {fit_v_sun:>18.15} + {fit_v_sun_ep:>17.15} - {fit_v_sun_em:>17.15}
                  W_sun: {w_sun:>18.15} -> {fit_w_sun:>18.15} + {fit_w_sun_ep:>17.15} - {fit_w_sun_em:>17.15}
                sigma_R: {sigma_r_g:>18.15} -> {fit_sigma_r_g:>18.15} + {fit_sigma_r_g_ep:>17.15} - {fit_sigma_r_g_em:>17.15}
            sigma_theta: {sigma_theta:>18.15} -> {fit_sigma_theta:>18.15} + {fit_sigma_theta_ep:>17.15} - {fit_sigma_theta_em:>17.15}
                sigma_Z: {sigma_z:>18.15} -> {fit_sigma_z:>18.15} + {fit_sigma_z_ep:>17.15} - {fit_sigma_z_em:>17.15}

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
                r_0 = params.r_0,
                omega_0 = params.omega_0,
                a = params.a,
                u_sun = params.u_sun,
                v_sun = params.v_sun,
                w_sun = params.w_sun,
                sigma_r_g = params.sigma_r_g,
                sigma_theta = params.sigma_theta,
                sigma_z = params.sigma_z,
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
