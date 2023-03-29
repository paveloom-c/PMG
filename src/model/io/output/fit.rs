//! Fit of the model

use crate::model::{Model, Params};

use core::fmt::{Debug, Display};
use core::iter::Sum;
use std::path::Path;

use anyhow::Result;
use indoc::formatdoc;
use num::{traits::FloatConst, Float};
use rand::distributions::uniform::SampleUniform;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;

/// Name of the files
const NAME: &str = "fit";

/// Output data record
#[derive(Serialize)]
struct Record<F: Float + Debug> {
    /// Galactocentric distance to the Sun (kpc)
    #[serde(rename = "R_0")]
    pub r_0: F,
    /// Circular velocity of the Sun at R = R_0 (km/s/kpc)
    pub omega_0: F,
    /// Oort's A constant (km/s/kpc)
    #[serde(rename = "A")]
    pub a: F,
    /// Standard Solar Motion toward GC (km/s)
    pub u_sun_standard: F,
    /// Standard Solar Motion toward l = 90 degrees (km/s)
    pub v_sun_standard: F,
    /// Standard Solar Motion toward NGP (km/s)
    pub w_sun_standard: F,
    /// Radial component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_r: F,
    /// Azimuthal component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_theta: F,
    /// Vertical component of the ellipsoid of natural standard deviations (km/s)
    pub sigma_z: F,
}

#[allow(clippy::many_single_char_names)]
impl<F> From<&Params<F>> for Record<F>
where
    F: Float + Default + Display + Debug,
{
    fn from(params: &Params<F>) -> Self {
        Self {
            r_0: params.r_0,
            omega_0: params.omega_0,
            a: params.a,
            u_sun_standard: params.u_sun_standard,
            v_sun_standard: params.v_sun_standard,
            w_sun_standard: params.w_sun_standard,
            sigma_r: params.sigma_r,
            sigma_theta: params.sigma_theta,
            sigma_z: params.sigma_z,
        }
    }
}

impl<F> Model<F>
where
    F: Float
        + FloatConst
        + SampleUniform
        + Default
        + Debug
        + Display
        + Serialize
        + Sync
        + Send
        + Sum,
    StandardNormal: Distribution<F>,
{
    /// Serialize the fitted parameters
    #[allow(clippy::non_ascii_literal)]
    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    pub(in crate::model) fn serialize_to_fit(&self, dat_dir: &Path, bin_dir: &Path) -> Result<()> {
        // Prepare a header
        let header = formatdoc!(
            "
            # Fitted parameters
            #
            # Descriptions:
            #
            # 1 R_0: Galactocentric distance to the Sun [kpc]
            # 2 omega_0: Circular velocity of the Sun at R = R_0 [km/s/kpc]
            # 3 A: Oort's A constant [km/s/kpc]
            # 4 u_sun_standard: Standard Solar Motion toward GC [km/s]
            # 5 v_sun_standard: Standard Solar Motion toward l = 90 degrees [km/s]
            # 6 w_sun_standard: Standard Solar Motion toward NGP [km/s]
            # 7 sigma_r: Radial component of the ellipsoid of natural standard deviations [km/s]
            # 8 sigma_theta: Azimuthal component of the ellipsoid of natural standard deviations [km/s]
            # 9 sigma_z: Vertical component of the ellipsoid of natural standard deviations [km/s]
            #
            # The first row contains the initial parameters, the second one — the fitted ones.
            #
            # Constant parameters used:
            #
            # The right ascension of the north galactic pole (HMS angle -> radians)
            # Source: Reid et al. (2009)
            # ALPHA_NGP: {alpha_ngp} [12:51:26.2817]
            #
            # The declination of the north galactic pole (DMS angle -> radians)
            # Source: Reid et al. (2009)
            # DELTA_NGP: {delta_ngp} [27:07:42.013]
            #
            # The longitude of the north celestial pole (decimal degrees angle -> radians)
            # Source: Reid et al. (2009)
            # L_NCP: {l_ncp} [122.932]
            #
            # Linear velocities units conversion coefficient
            # Sources: Gromov, Nikiforov (2016)
            # K: {k}
            #
            ",
            alpha_ngp = self.params.alpha_ngp,
            delta_ngp = self.params.delta_ngp,
            l_ncp = self.params.l_ncp,
            k = self.params.k,
        );
        super::serialize_to(
            dat_dir,
            bin_dir,
            NAME,
            &header,
            vec![
                Record::from(&self.params),
                Record::from(self.fit_params.as_ref().unwrap()),
            ],
        )
    }
}
