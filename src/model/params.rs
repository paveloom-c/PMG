//! Model parameters

use super::io::output;
use super::Model;

use core::fmt::{Debug, Display};
use std::path::Path;

use anyhow::Result;
use indoc::formatdoc;
use num::Float;
use serde::Serialize;

/// Model parameters
#[derive(Default, Debug, Clone, Serialize)]
pub struct Params<F> {
    /// Galactocentric distance to the Sun (kpc)
    #[serde(rename = "R_0")]
    pub r_0: F,
    /// Circular velocity of the Sun at R = R_0 (km/s/kpc)
    pub omega_0: F,
    /// Oort's A constant (km/s/kpc)
    #[serde(rename = "A")]
    pub a: F,
    /// Peculiar motion of the Sun toward GC (km/s)
    #[serde(rename = "U_sun")]
    pub u_sun: F,
    /// Peculiar motion of the Sun toward l = 90 degrees (km/s)
    #[serde(rename = "V_sun")]
    pub v_sun: F,
    /// Peculiar motion of the Sun toward NGP (km/s)
    #[serde(rename = "W_sun")]
    pub w_sun: F,
    /// Radial component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_r: F,
    /// Azimuthal component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_theta: F,
    /// Vertical component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_z: F,
    /// Linear rotation velocity of the Sun without peculiar motion (km/s)
    pub theta_0: F,
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
        self.sigma_r = p[6];
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
            self.sigma_r,
            self.sigma_theta,
            self.sigma_z,
        ]
        .to_vec()
    }
}

impl<F> Model<F> {
    /// Serialize the fitted parameters
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub(in crate::model) fn serialize_to_fit_params(
        &self,
        dat_dir: &Path,
        bin_dir: &Path,
    ) -> Result<()>
    where
        F: Float + Debug + Display + Serialize,
    {
        // Prepare a header
        let header = formatdoc!(
            "
            # Fit of the model (parameters)
            {sample_description}
            # Descriptions:
            #
            # 01 R_0: Galactocentric distance to the Sun [kpc]
            # 02 omega_0: Circular velocity of the Sun at R = R_0 [km/s/kpc]
            # 03 A: Oort's A constant [km/s/kpc]
            # 04 U_sun: Peculiar motion of the Sun toward GC [km/s]
            # 05 V_sun: Peculiar motion of the Sun toward l = 90 degrees [km/s]
            # 06 W_sun: Peculiar motion of the Sun toward NGP [km/s]
            # 07 sigma_r: Radial component of the ellipsoid of natural standard deviations [km/s]
            # 08 sigma_theta: Azimuthal component of the ellipsoid of natural standard deviations [km/s]
            # 09 sigma_z: Vertical component of the ellipsoid of natural standard deviations [km/s]
            # 10 theta_0: Linear rotation velocity of the Sun without peculiar motion [km/s]
            # 11 theta_sun: Linear rotation velocity of the Sun [km/s]
            #
            # Note that only the first 9 parameters were optimized.
            # The rest are derived from the results.
            #
            # The first row in the output contains the initial
            # parameters, the second one -- the fitted ones.
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
            alpha_ngp = self.params.alpha_ngp,
            delta_ngp = self.params.delta_ngp,
            l_ncp = self.params.l_ncp,
            k = self.params.k,
            u_sun_standard = self.params.u_sun_standard,
            v_sun_standard = self.params.v_sun_standard,
            w_sun_standard = self.params.w_sun_standard,
        );
        let records = vec![&self.params, self.fit_params.as_ref().unwrap()];
        output::serialize_to(dat_dir, bin_dir, "fit_params", &header, &records)
    }
}
