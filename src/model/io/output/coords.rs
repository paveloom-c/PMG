//! Galactic heliocentric coordinates of the objects

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
const NAME: &str = "coords";

/// Output data record
#[derive(Serialize)]
struct Record<'a, F: Float + Debug> {
    /// Name
    name: &'a str,
    /// Type of the object
    #[serde(rename = "type")]
    obj_type: &'a str,
    /// Source of the data
    source: &'a str,
    /// Longitude (deg)
    l: F,
    /// Latitude (deg)
    b: F,
    /// X coordinate (kpc)
    #[serde(rename = "X")]
    x: F,
    /// Plus uncertainty in `x` (kpc)
    #[serde(rename = "ep_X")]
    e_p_x: F,
    /// Minus uncertainty in `x` (kpc)
    #[serde(rename = "em_X")]
    e_m_x: F,
    /// Y coordinate (kpc)
    #[serde(rename = "Y")]
    y: F,
    /// Plus uncertainty in `y` (kpc)
    #[serde(rename = "ep_Y")]
    e_p_y: F,
    /// Minus uncertainty in `y` (kpc)
    #[serde(rename = "em_Y")]
    e_m_y: F,
    /// Z coordinate (kpc)
    #[serde(rename = "Z")]
    z: F,
    /// Plus uncertainty in `z` (kpc)
    #[serde(rename = "ep_Z")]
    e_p_z: F,
    /// Minus uncertainty in `z` (kpc)
    #[serde(rename = "em_Z")]
    e_m_z: F,
    /// Heliocentric distance (kpc)
    #[serde(rename = "r")]
    r_h: F,
    /// Plus uncertainty in `r_h` (kpc)
    #[serde(rename = "ep_r")]
    e_p_r_h: F,
    /// Minus uncertainty in `r_h` (kpc)
    #[serde(rename = "em_r")]
    e_m_r_h: F,
    /// Galactocentric distance (kpc)
    #[serde(rename = "R")]
    r_g: F,
    /// Plus uncertainty in `r_g` (kpc)
    #[serde(rename = "ep_R")]
    e_p_r_g: F,
    /// Minus uncertainty in `r_g` (kpc)
    #[serde(rename = "em_R")]
    e_m_r_g: F,
    /// Proper motion on longitude (mas/yr)
    #[serde(rename = "mu_l")]
    mu_l: F,
    /// Proper motion on latitude (mas/yr)
    #[serde(rename = "mu_b")]
    mu_b: F,
    /// Heliocentric velocity in distance (km/s)
    #[serde(rename = "V_r")]
    v_r: F,
    /// Heliocentric velocity in longitude (km/s)
    #[serde(rename = "V_l")]
    v_l: F,
    /// Plus uncertainty in `v_l` (km/s)
    #[serde(rename = "ep_V_l")]
    e_p_v_l: F,
    /// Minus uncertainty in `v_l` (km/s)
    #[serde(rename = "em_V_l")]
    e_m_v_l: F,
    /// Heliocentric velocity in latitude (km/s)
    #[serde(rename = "V_b")]
    v_b: F,
    /// Plus uncertainty in `v_b` (km/s)
    #[serde(rename = "ep_V_b")]
    e_p_v_b: F,
    /// Minus uncertainty in `v_b` (km/s)
    #[serde(rename = "em_V_b")]
    e_m_v_b: F,
    /// U coordinate (km/s)
    #[serde(rename = "U")]
    u: F,
    /// Plus uncertainty in `u` (km/s)
    #[serde(rename = "ep_U")]
    e_p_u: F,
    /// Minus uncertainty in `u` (km/s)
    #[serde(rename = "em_U")]
    e_m_u: F,
    /// V coordinate (km/s)
    #[serde(rename = "V")]
    v: F,
    /// Plus uncertainty in `v` (km/s)
    #[serde(rename = "ep_V")]
    e_p_v: F,
    /// Minus uncertainty in `v` (km/s)
    #[serde(rename = "em_V")]
    e_m_v: F,
    /// W coordinate (km/s)
    #[serde(rename = "W")]
    w: F,
    /// Plus uncertainty in `w` (km/s)
    #[serde(rename = "ep_W")]
    e_p_w: F,
    /// Minus uncertainty in `w` (km/s)
    #[serde(rename = "em_W")]
    e_m_w: F,
}

#[allow(clippy::many_single_char_names)]
impl<'a, F> TryFrom<&'a Object<F>> for Record<'a, F>
where
    F: Float + FloatConst + Default + Display + Debug,
{
    type Error = anyhow::Error;

    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    fn try_from(object: &'a Object<F>) -> Result<Self> {
        let name = object.name.as_ref().unwrap();
        let obj_type = object.obj_type.as_ref().unwrap();
        let source = object.source.as_ref().unwrap();
        let l = object.l.unwrap();
        let b = object.b.unwrap();
        let r_h = object.r_h.as_ref().unwrap();
        let r_g = object.r_g.as_ref().unwrap();
        let x = object.x.as_ref().unwrap();
        let y = object.y.as_ref().unwrap();
        let z = object.z.as_ref().unwrap();
        let mu_l = object.mu_l.unwrap();
        let mu_b = object.mu_b.unwrap();
        let v_r = object.v_r.unwrap();
        let v_l = object.v_l.as_ref().unwrap();
        let v_b = object.v_b.as_ref().unwrap();
        let u = object.u.as_ref().unwrap();
        let v = object.v.as_ref().unwrap();
        let w = object.w.as_ref().unwrap();
        Ok(Self {
            name,
            obj_type,
            source,
            l: l.to_degrees(),
            b: b.to_degrees(),
            x: x.v,
            e_p_x: x.e_p,
            e_m_x: x.e_m,
            y: y.v,
            e_p_y: y.e_p,
            e_m_y: y.e_m,
            z: z.v,
            e_p_z: z.e_p,
            e_m_z: z.e_m,
            r_h: r_h.v,
            e_p_r_h: r_h.e_p,
            e_m_r_h: r_h.e_m,
            r_g: r_g.v,
            e_p_r_g: r_g.e_p,
            e_m_r_g: r_g.e_m,
            mu_l,
            mu_b,
            v_r,
            v_l: v_l.v,
            e_p_v_l: v_l.e_p,
            e_m_v_l: v_l.e_m,
            v_b: v_b.v,
            e_p_v_b: v_b.e_p,
            e_m_v_b: v_b.e_m,
            u: u.v,
            e_p_u: u.e_p,
            e_m_u: u.e_m,
            v: v.v,
            e_p_v: v.e_p,
            e_m_v: v.e_m,
            w: w.v,
            e_p_w: w.e_p,
            e_m_w: w.e_m,
        })
    }
}

/// Output data records
type Records<'a, F> = Vec<Record<'a, F>>;

impl<'a, F> TryFrom<&'a Model<F>> for Records<'a, F>
where
    F: Float + FloatConst + SampleUniform + Default + Display + Debug + Send + Sync,
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
    F: Float + FloatConst + SampleUniform + Default + Debug + Display + Serialize + Send + Sync,
    StandardNormal: Distribution<F>,
{
    /// Serialize the Galactic heliocentric coordinates of the objects
    pub(in crate::model) fn serialize_to_coords(
        &self,
        dat_dir: &Path,
        bin_dir: &Path,
    ) -> Result<()> {
        // Prepare a header
        let header = formatdoc!(
            "
            # Galactic heliocentric coordinates of the objects
            #
            # Descriptions:
            #
            # 1  name: Name of the object
            # 2  type: Type of the object
            # 3  source: Source of the data
            # 4  l: Longitude [deg]
            # 5  b: Latitude [deg]
            # 6  X: X coordinate [kpc]
            # 7  ep_X: Plus uncertainty in `X` [kpc]
            # 8  em_X: Minus uncertainty in `X` [kpc]
            # 9  Y: Y coordinate [kpc]
            # 10 ep_Y: Plus uncertainty in `Y` [kpc]
            # 11 em_Y: Minus uncertainty in `Y` [kpc]
            # 12 Z: Z coordinate [kpc]
            # 13 ep_Z: Plus uncertainty in `Z` [kpc]
            # 14 em_Z: Minus uncertainty in `Z` [kpc]
            # 15 r: Heliocentric distance [kpc]
            # 16 ep_r: Plus uncertainty in `r` [kpc]
            # 17 em_r: Minus uncertainty in `r` [kpc]
            # 18 R: Galactocentric distance [kpc]
            # 19 ep_R: Plus uncertainty in `R` [kpc]
            # 20 em_R: Minus uncertainty in `R` [kpc]
            # 21 mu_l: Proper motion in longitude [mas/yr]
            # 22 mu_b: Proper motion in latitude [mas/yr]
            # 23 V_r: Heliocentric velocity in distance [km/s]
            # 24 V_l: Heliocentric velocity in longitude [km/s]
            # 25 ep_V_l: Plus uncertainty in `V_l` [km/s]
            # 26 em_V_l: Minus uncertainty in `V_l` [km/s]
            # 27 V_b: Heliocentric velocity in latitude [km/s]
            # 28 ep_V_b: Plus uncertainty in `V_b` [km/s]
            # 29 em_V_b: Minus uncertainty in `V_b` [km/s]
            # 30 U: U coordinate [km/s]
            # 31 ep_U: Plus uncertainty in `U` [km/s]
            # 32 em_U: Minus uncertainty in `U` [km/s]
            # 33 V: V coordinate [km/s]
            # 34 ep_V: Plus uncertainty in `V` [km/s]
            # 35 em_V: Minus uncertainty in `V` [km/s]
            # 36 W: W coordinate [km/s]
            # 37 ep_W: Plus uncertainty in `W` [km/s]
            # 38 em_W: Minus uncertainty in `W` [km/s]
            #
            # Uncertainties come from assuming maximum and minimum values of the parallax.
            # Note that they are not independent from each other and can be negative here.
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
            # R_0: {r_0}
            #
            ",
            alpha_ngp = self.params.alpha_ngp,
            delta_ngp = self.params.delta_ngp,
            l_ncp = self.params.l_ncp,
            r_0 = self.params.r_0,
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
