//! Rotation curve

use crate::model::{Model, Object};

use core::fmt::{Debug, Display};
use std::path::Path;

use anyhow::{Context, Result};
use indoc::formatdoc;
use num::{traits::FloatConst, Float};
use rand::distributions::uniform::SampleUniform;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;

/// Name of the files
const NAME: &str = "rotcurve";

/// Output data record
#[derive(Serialize)]
struct Record<'a, F: Float + Debug> {
    /// Name
    name: &'a String,
    /// Azimuthal velocity (km/s)
    theta: F,
    /// Plus uncertainty in `theta` (km/s)
    ep_theta: F,
    /// Plus uncertainty in `theta` (km/s)
    em_theta: F,
    /// Velocity uncertainty in `theta` (km/s)
    evel_theta: F,
    /// Galactocentric distance (kpc)
    #[serde(rename = "R")]
    r_g: F,
    /// Plus uncertainty in `r_g` (km/s)
    #[serde(rename = "ep_R")]
    e_p_r_g: F,
    /// Minus uncertainty in `r_g` (km/s)
    #[serde(rename = "em_R")]
    e_m_r_g: F,
    /// Type of the object
    #[serde(rename = "type")]
    obj_type: &'a String,
    /// Source of the data
    source: &'a String,
}

#[allow(clippy::many_single_char_names)]
impl<'a, F> TryFrom<&'a Object<F>> for Record<'a, F>
where
    F: Float + FloatConst + Default + Display + Debug,
{
    type Error = anyhow::Error;

    fn try_from(object: &'a Object<F>) -> Result<Self> {
        let name = object.name()?;
        let (theta, r_g) = object.rotation_c()?.into();
        let obj_type = object.obj_type()?;
        let source = object.source()?;
        Ok(Self {
            name,
            theta: theta.measurement.v,
            ep_theta: theta.measurement.e_p,
            em_theta: theta.measurement.e_m,
            evel_theta: theta.e_vel,
            r_g: r_g.v,
            e_p_r_g: r_g.e_p,
            e_m_r_g: r_g.e_m,
            obj_type,
            source,
        })
    }
}

/// Output data records
type Records<'a, F> = Vec<Record<'a, F>>;

impl<'a, F> TryFrom<&'a Model<F>> for Records<'a, F>
where
    F: Float + FloatConst + SampleUniform + Default + Display + Debug + Sync,
    StandardNormal: Distribution<F>,
{
    type Error = anyhow::Error;

    fn try_from(model: &'a Model<F>) -> Result<Self> {
        model
            .objects
            .iter()
            .map(|object| {
                Record::try_from(object)
                    .with_context(|| "Couldn't construct a record from the object")
            })
            .collect()
    }
}

impl<F> Model<F>
where
    F: Float + FloatConst + SampleUniform + Default + Debug + Display + Serialize + Sync,
    StandardNormal: Distribution<F>,
{
    /// Serialize the rotation curve
    pub(in crate::model) fn serialize_to_rotcurve(
        &self,
        dat_dir: &Path,
        bin_dir: &Path,
    ) -> Result<()> {
        // Prepare a header
        let header = formatdoc!(
            "
            # Rotation curve
            #
            # Descriptions:
            #
            # 1 name: Name of the object
            # 2 theta: Azimuthal velocity [km/s]
            # 3 ep_theta: Plus uncertainty in `theta` [km/s]
            # 4 em_theta: Minus uncertainty in `theta` [km/s]
            # 5 evel_theta: Velocity uncertainty in `theta` [km/s]
            # 6 R: Galactocentric distance [kpc]
            # 7 ep_R: Plus uncertainty in `R` [kpc]
            # 8 em_R: Minus uncertainty in `R` [kpc]
            # 9 type: Type of the object
            # 10 source: Source of the data
            #
            # Uncertainties in the distance come from assuming maximum and minimum
            # values of the parallax. Note that they can be negative here.
            #
            # Parameters used:
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
            # Galactocentric distance to the Sun (kpc)
            # Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
            # R_0: {r_0}
            #
            # Standard Solar Motion toward GC (km/s)
            # Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
            # U_SUN_STANDARD: {u_sun_standard}
            #
            # Peculiar motion locally toward GC (km/s)
            # Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
            # U_SUN: {u_sun}
            #
            # Standard Solar Motion toward l = 90 degrees (km/s)
            # Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
            # V_SUN_STANDARD: {v_sun_standard}
            #
            # Full circular velocity of the Sun (km/s)
            # Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
            # THETA_SUN: {theta_sun}
            #
            # Standard Solar Motion toward NGP (km/s)
            # Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
            # W_SUN_STANDARD: {w_sun_standard}
            #
            # Linear velocities units conversion coefficient
            # Sources: Gromov, Nikiforov (2016)
            # K: {k}
            #
            ",
            alpha_ngp = self.params.alpha_ngp,
            delta_ngp = self.params.delta_ngp,
            l_ncp = self.params.l_ncp,
            r_0 = self.params.r_0,
            u_sun_standard = self.params.u_sun_standard,
            u_sun = self.params.u_sun,
            v_sun_standard = self.params.v_sun_standard,
            theta_sun = self.params.theta_sun,
            w_sun_standard = self.params.w_sun_standard,
            k = self.params.k,
        );
        super::serialize_to(
            dat_dir,
            bin_dir,
            NAME,
            &header,
            Records::try_from(self).with_context(|| "Couldn't construct records from the model")?,
        )
    }
}
