//! Model parameters

use super::io::output;
use super::Model;

use core::fmt::{Debug, Display};
use core::ops::Range;
use std::path::Path;

use anyhow::Result;
use indoc::formatdoc;
use num::Float;
use serde::Serialize;
use simulated_annealing::Point;

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
    /// Standard Solar Motion toward GC (km/s)
    #[serde(rename = "U_sun_standard")]
    pub u_sun_standard: F,
    /// Standard Solar Motion toward l = 90 degrees (km/s)
    #[serde(rename = "V_sun_standard")]
    pub v_sun_standard: F,
    /// Standard Solar Motion toward NGP (km/s)
    #[serde(rename = "W_sun_standard")]
    pub w_sun_standard: F,
    /// Radial component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_r: F,
    /// Azimuthal component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_theta: F,
    /// Vertical component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_z: F,
    /// The right ascension of the north galactic pole (radians)
    pub alpha_ngp: F,
    /// The declination of the north galactic pole (radians)
    pub delta_ngp: F,
    /// The longitude of the north celestial pole (radians)
    pub l_ncp: F,
    /// Linear velocities units conversion coefficient
    pub k: F,
    /// Full circular velocity of the Sun (km/s)
    pub theta_sun: F,
    /// Peculiar motion locally toward GC (km/s)
    #[serde(rename = "U_sun")]
    pub u_sun: F,
}

impl<F> Params<F> {
    /// Update the parameters with the point in the parameter space
    ///
    /// Note that not all fields are updated, but only those needed for fitting
    pub fn update_with(&mut self, p: &Point<F, 9>)
    where
        F: Float + Debug,
    {
        self.r_0 = p[0];
        self.omega_0 = p[1];
        self.a = p[2];
        self.u_sun_standard = p[3];
        self.v_sun_standard = p[4];
        self.w_sun_standard = p[5];
        self.sigma_r = p[6];
        self.sigma_theta = p[7];
        self.sigma_z = p[8];
    }
    /// Construct a point in the parameter space from the parameters
    ///
    /// Note that not all fields are used, but only those needed for fitting
    pub fn to_point(&self) -> Point<F, 9>
    where
        F: Float + Debug,
    {
        [
            self.r_0,
            self.omega_0,
            self.a,
            self.u_sun_standard,
            self.v_sun_standard,
            self.w_sun_standard,
            self.sigma_r,
            self.sigma_theta,
            self.sigma_z,
        ]
    }
    /// Return bounds to the parameters
    ///
    /// Note that not all fields are used, but only those needed for fitting
    pub fn bounds() -> [Range<F>; 9]
    where
        F: Float + Debug,
    {
        [
            F::zero()..F::infinity(),
            F::zero()..F::infinity(),
            F::zero()..F::infinity(),
            F::zero()..F::infinity(),
            F::zero()..F::infinity(),
            F::zero()..F::infinity(),
            F::zero()..F::infinity(),
            F::zero()..F::infinity(),
            F::zero()..F::infinity(),
        ]
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
            #
            # Descriptions:
            #
            # 1 R_0: Galactocentric distance to the Sun [kpc]
            # 2 omega_0: Circular velocity of the Sun at R = R_0 [km/s/kpc]
            # 3 A: Oort's A constant [km/s/kpc]
            # 4 U_sun_standard: Standard Solar Motion toward GC [km/s]
            # 5 V_sun_standard: Standard Solar Motion toward l = 90 degrees [km/s]
            # 6 W_sun_standard: Standard Solar Motion toward NGP [km/s]
            # 7 sigma_r: Radial component of the ellipsoid of natural standard deviations [km/s]
            # 8 sigma_theta: Azimuthal component of the ellipsoid of natural standard deviations [km/s]
            # 9 sigma_z: Vertical component of the ellipsoid of natural standard deviations [km/s]
            # 10 alpha_ngp: The right ascension of the north galactic pole [HMS angle -> radians]
            # 11 delta_ngp: The declination of the north galactic pole [DMS angle -> radians]
            # 12 l_ncp: The longitude of the north celestial pole [decimal degrees angle -> radians]
            # 13 k: Linear velocities units conversion coefficient
            # 14 theta_sun: Full circular velocity of the Sun [km/s]
            # 15 u_sun: Peculiar motion locally toward GC [km/s]
            #
            # Note that only first 9 parameters were optimized.
            #
            # The first row in the output contains the initial
            # parameters, the second one -- the fitted ones.
            #
            ",
        );
        let records = vec![&self.params, self.fit_params.as_ref().unwrap()];
        output::serialize_to(dat_dir, bin_dir, "fit_params", &header, &records)
    }
}
