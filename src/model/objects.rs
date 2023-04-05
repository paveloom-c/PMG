//! Data objects

mod equatorial_spherical;
mod galactic_cartesian;
mod galactic_spherical;
mod mu;
mod r_g;
mod theta;

use super::io::{input, output};
use super::{Model, Params};

use core::fmt::{Debug, Display};
use core::str::FromStr;
use std::error::Error;
use std::path::Path;

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use indoc::formatdoc;
use num::Float;
use numeric_literals::replace_float_literals;
use serde::{de::DeserializeOwned, Serialize, Serializer};

/// Data objects
pub type Objects<F> = Vec<Object<F>>;

/// Serialize the value of an option only if it's a `Some` variant
#[allow(clippy::unwrap_used)]
pub fn serialize_option<T, S>(option: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    option.as_ref().unwrap().serialize(serializer)
}

/// Data object
///
/// Computed plus and minus values are values that inherit
/// uncertainties only from the parallax error.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(bound = "F: Serialize")]
pub struct Object<F> {
    /// Name of the object
    #[serde(serialize_with = "serialize_option")]
    pub name: Option<String>,
    /// Type of the object
    #[serde(rename = "type")]
    #[serde(serialize_with = "serialize_option")]
    pub obj_type: Option<String>,
    /// Source of the data
    #[serde(serialize_with = "serialize_option")]
    pub source: Option<String>,
    /// Right ascension (radians)
    #[serde(serialize_with = "serialize_option")]
    pub alpha: Option<F>,
    /// Declination (radians)
    #[serde(serialize_with = "serialize_option")]
    pub delta: Option<F>,
    /// Parallax (mas)
    #[serde(serialize_with = "serialize_option")]
    pub par: Option<F>,
    /// Uncertainty in `par` (mas)
    #[serde(serialize_with = "serialize_option")]
    pub par_e: Option<F>,
    /// Plus value of `par`
    #[serde(serialize_with = "serialize_option")]
    pub par_p: Option<F>,
    /// Minus value of `par`
    #[serde(serialize_with = "serialize_option")]
    pub par_m: Option<F>,
    /// Local Standard of Rest velocity (km/s)
    #[serde(serialize_with = "serialize_option")]
    #[serde(rename = "V_lsr")]
    pub v_lsr: Option<F>,
    /// Uncertainty in `v_lsr` (km/s)
    #[serde(serialize_with = "serialize_option")]
    #[serde(rename = "V_lsr_e")]
    pub v_lsr_e: Option<F>,
    /// Plus value of `v_lsr` (km/s)
    #[serde(serialize_with = "serialize_option")]
    #[serde(rename = "V_lsr_p")]
    pub v_lsr_p: Option<F>,
    /// Minus value of `v_lsr` (km/s)
    #[serde(serialize_with = "serialize_option")]
    #[serde(rename = "V_lsr_m")]
    pub v_lsr_m: Option<F>,
    /// Eastward proper motion (mas/yr)
    #[serde(serialize_with = "serialize_option")]
    pub mu_x: Option<F>,
    /// Uncertainty in `mu_x` (mas/yr)
    #[serde(serialize_with = "serialize_option")]
    pub mu_x_e: Option<F>,
    /// Plus value of `mu_x` (mas/yr)
    #[serde(serialize_with = "serialize_option")]
    pub mu_x_p: Option<F>,
    /// Minus value of `mu_x` (mas/yr)
    #[serde(serialize_with = "serialize_option")]
    pub mu_x_m: Option<F>,
    /// Northward proper motion (mas/yr)
    #[serde(serialize_with = "serialize_option")]
    pub mu_y: Option<F>,
    /// Uncertainty in `mu_y` (mas/yr)
    #[serde(serialize_with = "serialize_option")]
    pub mu_y_e: Option<F>,
    /// Plus value of `mu_y` (mas/yr)
    #[serde(serialize_with = "serialize_option")]
    pub mu_y_p: Option<F>,
    /// Minus value of `mu_y` (mas/yr)
    #[serde(serialize_with = "serialize_option")]
    pub mu_y_m: Option<F>,
    /// Heliocentric distance (kpc)
    #[serde(rename = "r")]
    #[serde(serialize_with = "serialize_option")]
    pub r_h: Option<F>,
    /// Plus uncertainty in `r_h` (kpc)
    #[serde(rename = "r_ep")]
    #[serde(serialize_with = "serialize_option")]
    pub r_h_ep: Option<F>,
    /// Minus uncertainty in `r_h` (kpc)
    #[serde(rename = "r_em")]
    #[serde(serialize_with = "serialize_option")]
    pub r_h_em: Option<F>,
    /// Plus value of `r_h` (kpc)
    #[serde(rename = "r_p")]
    #[serde(serialize_with = "serialize_option")]
    pub r_h_p: Option<F>,
    /// Minus value of `r_h` (kpc)
    #[serde(rename = "r_m")]
    #[serde(serialize_with = "serialize_option")]
    pub r_h_m: Option<F>,
    /// Longitude (radians)
    #[serde(serialize_with = "serialize_option")]
    pub l: Option<F>,
    /// Latitude (radians)
    #[serde(serialize_with = "serialize_option")]
    pub b: Option<F>,
    /// Proper motion in longitude
    #[serde(serialize_with = "serialize_option")]
    pub mu_l: Option<F>,
    /// Proper motion in latitude
    #[serde(serialize_with = "serialize_option")]
    pub mu_b: Option<F>,
    /// Galactocentric distance (kpc)
    #[serde(rename = "R")]
    #[serde(serialize_with = "serialize_option")]
    pub r_g: Option<F>,
    /// Plus uncertainty in `r_g` (kpc)
    #[serde(rename = "R_ep")]
    #[serde(serialize_with = "serialize_option")]
    pub r_g_ep: Option<F>,
    /// Minus uncertainty in `r_g` (kpc)
    #[serde(rename = "R_em")]
    #[serde(serialize_with = "serialize_option")]
    pub r_g_em: Option<F>,
    /// Plus value of `r_g` (kpc)
    #[serde(rename = "R_p")]
    #[serde(serialize_with = "serialize_option")]
    pub r_g_p: Option<F>,
    /// Minus value of `r_g` (kpc)
    #[serde(rename = "R_m")]
    #[serde(serialize_with = "serialize_option")]
    pub r_g_m: Option<F>,
    /// X coordinate (kpc)
    #[serde(rename = "X")]
    #[serde(serialize_with = "serialize_option")]
    pub x: Option<F>,
    /// Plus uncertainty in `x` (kpc)
    #[serde(rename = "X_ep")]
    #[serde(serialize_with = "serialize_option")]
    pub x_ep: Option<F>,
    /// Minus uncertainty in `x` (kpc)
    #[serde(rename = "X_em")]
    #[serde(serialize_with = "serialize_option")]
    pub x_em: Option<F>,
    /// Plus value of `x` (kpc)
    #[serde(rename = "X_p")]
    #[serde(serialize_with = "serialize_option")]
    pub x_p: Option<F>,
    /// Minus value of `x` (kpc)
    #[serde(rename = "X_m")]
    #[serde(serialize_with = "serialize_option")]
    pub x_m: Option<F>,
    /// Y coordinate (kpc)
    #[serde(rename = "Y")]
    #[serde(serialize_with = "serialize_option")]
    pub y: Option<F>,
    /// Plus uncertainty in `y` (kpc)
    #[serde(rename = "Y_ep")]
    #[serde(serialize_with = "serialize_option")]
    pub y_ep: Option<F>,
    /// Minus uncertainty in `y` (kpc)
    #[serde(rename = "Y_em")]
    #[serde(serialize_with = "serialize_option")]
    pub y_em: Option<F>,
    /// Plus value of `y` (kpc)
    #[serde(rename = "Y_p")]
    #[serde(serialize_with = "serialize_option")]
    pub y_p: Option<F>,
    /// Minus value of `y` (kpc)
    #[serde(rename = "Y_m")]
    #[serde(serialize_with = "serialize_option")]
    pub y_m: Option<F>,
    /// Z coordinate (kpc)
    #[serde(rename = "Z")]
    #[serde(serialize_with = "serialize_option")]
    pub z: Option<F>,
    /// Plus uncertaintz in `z` (kpc)
    #[serde(rename = "Z_ep")]
    #[serde(serialize_with = "serialize_option")]
    pub z_ep: Option<F>,
    /// Minus uncertaintz in `z` (kpc)
    #[serde(rename = "Z_em")]
    #[serde(serialize_with = "serialize_option")]
    pub z_em: Option<F>,
    /// Plus value of `z` (kpc)
    #[serde(rename = "Z_p")]
    #[serde(serialize_with = "serialize_option")]
    pub z_p: Option<F>,
    /// Minus value of `z` (kpc)
    #[serde(rename = "Z_m")]
    #[serde(serialize_with = "serialize_option")]
    pub z_m: Option<F>,
    /// Azimuthal velocity (km/s)
    #[serde(serialize_with = "serialize_option")]
    pub theta: Option<F>,
    /// Plus uncertainty in `theta` (km/s)
    #[serde(serialize_with = "serialize_option")]
    pub theta_ep: Option<F>,
    /// Minus uncertainty in `theta` (km/s)
    #[serde(serialize_with = "serialize_option")]
    pub theta_em: Option<F>,
    /// Plus value of `theta` (km/s)
    #[serde(serialize_with = "serialize_option")]
    pub theta_p: Option<F>,
    /// Minus value of `theta` (km/s)
    #[serde(serialize_with = "serialize_option")]
    pub theta_m: Option<F>,
    /// Uncertainty in azimuthal velocity
    /// inherited from the velocities (km/s)
    #[serde(serialize_with = "serialize_option")]
    pub theta_evel: Option<F>,
    /// Heliocentric velocity in distance (km/s)
    #[serde(rename = "V_r")]
    #[serde(serialize_with = "serialize_option")]
    pub v_r: Option<F>,
    /// Heliocentric velocity in longitude (km/s)
    #[serde(rename = "V_l")]
    #[serde(serialize_with = "serialize_option")]
    pub v_l: Option<F>,
    /// Plus uncertainty in `v_l` (km/s)
    #[serde(rename = "V_l_ep")]
    #[serde(serialize_with = "serialize_option")]
    pub v_l_ep: Option<F>,
    /// Minus uncertainty in `v_l` (km/s)
    #[serde(rename = "V_l_em")]
    #[serde(serialize_with = "serialize_option")]
    pub v_l_em: Option<F>,
    /// Plus value of `v_l` (km/s)
    #[serde(rename = "V_l_p")]
    #[serde(serialize_with = "serialize_option")]
    pub v_l_p: Option<F>,
    /// Minus value of `v_l` (km/s)
    #[serde(rename = "V_l_m")]
    #[serde(serialize_with = "serialize_option")]
    pub v_l_m: Option<F>,
    /// Heliocentric velocity in latitude (km/s)
    #[serde(rename = "V_b")]
    #[serde(serialize_with = "serialize_option")]
    pub v_b: Option<F>,
    /// Plus uncertainty in `v_b` (km/s)
    #[serde(rename = "V_b_ep")]
    #[serde(serialize_with = "serialize_option")]
    pub v_b_ep: Option<F>,
    /// Minus uncertainty in `v_b` (km/s)
    #[serde(rename = "V_b_em")]
    #[serde(serialize_with = "serialize_option")]
    pub v_b_em: Option<F>,
    /// Plus value of `v_b` (km/s)
    #[serde(rename = "V_b_p")]
    #[serde(serialize_with = "serialize_option")]
    pub v_b_p: Option<F>,
    /// Minus value of `v_b` (km/s)
    #[serde(rename = "V_b_m")]
    #[serde(serialize_with = "serialize_option")]
    pub v_b_m: Option<F>,
    /// U coordinate (kpc)
    #[serde(rename = "U")]
    #[serde(serialize_with = "serialize_option")]
    pub u: Option<F>,
    /// Plus uncertainty in `u` (kpc)
    #[serde(rename = "U_ep")]
    #[serde(serialize_with = "serialize_option")]
    pub u_ep: Option<F>,
    /// Minus uncertainty in `u` (kpc)
    #[serde(rename = "U_em")]
    #[serde(serialize_with = "serialize_option")]
    pub u_em: Option<F>,
    /// Plus value of `u` (kpc)
    #[serde(rename = "U_p")]
    #[serde(serialize_with = "serialize_option")]
    pub u_p: Option<F>,
    /// Minus value of `u` (kpc)
    #[serde(rename = "U_m")]
    #[serde(serialize_with = "serialize_option")]
    pub u_m: Option<F>,
    /// V coordinate (kpc)
    #[serde(rename = "V")]
    #[serde(serialize_with = "serialize_option")]
    pub v: Option<F>,
    /// Plus uncertainty in `v` (km/s)
    #[serde(rename = "V_ep")]
    #[serde(serialize_with = "serialize_option")]
    pub v_ep: Option<F>,
    /// Minus uncertainty in `v` (km/s)
    #[serde(rename = "V_em")]
    #[serde(serialize_with = "serialize_option")]
    pub v_em: Option<F>,
    /// Plus value of `v` (km/s)
    #[serde(rename = "V_p")]
    #[serde(serialize_with = "serialize_option")]
    pub v_p: Option<F>,
    /// Minus value of `v` (km/s)
    #[serde(rename = "V_m")]
    #[serde(serialize_with = "serialize_option")]
    pub v_m: Option<F>,
    /// W coordinate (kpc)
    #[serde(rename = "W")]
    #[serde(serialize_with = "serialize_option")]
    pub w: Option<F>,
    /// Plus uncertainty in `w` (km/s)
    #[serde(rename = "W_ep")]
    #[serde(serialize_with = "serialize_option")]
    pub w_ep: Option<F>,
    /// Minus uncertainty in `w` (km/s)
    #[serde(rename = "W_em")]
    #[serde(serialize_with = "serialize_option")]
    pub w_em: Option<F>,
    /// Plus value of `w` (km/s)
    #[serde(rename = "W_p")]
    #[serde(serialize_with = "serialize_option")]
    pub w_p: Option<F>,
    /// Minus value of `w` (km/s)
    #[serde(rename = "W_m")]
    #[serde(serialize_with = "serialize_option")]
    pub w_m: Option<F>,
}

impl<F> Object<F> {
    /// Perform per-object computations
    pub(in crate::model) fn compute(&mut self, params: &Params<F>)
    where
        F: Float + Debug + Default,
    {
        self.compute_l_b(params);
        self.compute_r_h();
        self.compute_r_g(params);
        self.compute_mu_l_mu_b(params);
        self.compute_v_r(params);
        self.compute_v_l_v_b(params);
        self.compute_u_v_w();
        self.compute_x_y_z();
        self.compute_theta(params);
        self.compute_e_vel_theta(params);
    }
}

impl<F> TryFrom<input::Record<F>> for Object<F>
where
    F: Float + Default + Debug + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    #[allow(clippy::unwrap_in_result)]
    #[allow(clippy::unwrap_used)]
    #[replace_float_literals(F::from(literal).unwrap())]
    fn try_from(record: input::Record<F>) -> Result<Self> {
        let mut object = Self::default();
        object.try_parse_alpha(&record)?;
        object.try_parse_delta(&record)?;
        object.name = Some(record.name);
        object.obj_type = Some(record.obj_type);
        object.source = Some(record.source);
        object.par = Some(record.par);
        object.par_e = Some(record.par_e);
        object.par_p = Some(record.par + record.par_e);
        // In some cases the uncertainty of the value can be greater than
        // the nominal value, hence leading to non-positive results in this
        // subtraction. We avoid this here since there is no such thing
        // as a non-positive parallax. Instead, we assume the distance to be
        // a finite, but sufficiently big value.
        object.par_m = Some({
            let v_l = record.par - record.par_e;
            if v_l > 0.0 {
                v_l
            } else {
                1. / 50.
            }
        });
        object.v_lsr = Some(record.v_lsr);
        object.v_lsr_e = Some(record.v_lsr_e);
        object.v_lsr_p = Some(record.v_lsr + record.v_lsr_e);
        object.v_lsr_m = Some(record.v_lsr - record.v_lsr_e);
        object.mu_x = Some(record.mu_x);
        object.mu_x_e = Some(record.mu_x_e);
        object.mu_x_p = Some(record.mu_x + record.mu_x_e);
        object.mu_x_m = Some(record.mu_x - record.mu_x_e);
        object.mu_y = Some(record.mu_y);
        object.mu_y_e = Some(record.mu_y_e);
        object.mu_y_p = Some(record.mu_y + record.mu_y_e);
        object.mu_y_m = Some(record.mu_y - record.mu_y_e);
        Ok(object)
    }
}

/// Parse a record into an object
fn deserialize<F>(result: Result<input::Record<F>, csv::Error>) -> Result<Object<F>>
where
    F: Float + Default + Debug + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    // Try to deserialize the record
    let record: input::Record<F> = result.with_context(|| "Couldn't deserialize a record")?;
    // Parse an object from the record
    let object =
        Object::try_from(record).with_context(|| "Couldn't parse a record into an object")?;
    Ok(object)
}

impl<F> Model<F> {
    /// Try to load data from the path
    pub fn try_load_data_from(&mut self, path: &Path) -> Result<()>
    where
        F: Float + Default + Debug + DeserializeOwned + FromStr,
        <F as FromStr>::Err: Error + Send + Sync + 'static,
    {
        // Create a CSV reader
        let mut rdr = ReaderBuilder::default()
            .delimiter(b' ')
            .comment(Some(b'#'))
            .from_path(path)
            .with_context(|| format!("Couldn't read from the file {path:?}"))?;
        // Try to collect objects
        self.objects = rdr
            .deserialize()
            .map(deserialize)
            .collect::<Result<Vec<Object<F>>>>()
            .with_context(|| format!("Couldn't get objects from the file {path:?}"))?;
        Ok(())
    }
    /// Serialize the per-object data
    #[allow(clippy::too_many_lines)]
    pub(in crate::model) fn serialize_to_objects(
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
            # Per-object data
            {sample_description}
            # Descriptions:
            #
            # 01 name: Name of the object
            # 02 type: Type of the object
            # 03 source: Source of the data
            # 04 alpha: Right ascension [radians]
            # 05 delta: Declination [radians]
            # 06 par: Parallax [mas]
            # 07 par_e: Uncertainty in `par` [mas]
            # 08 par_p: Plus value of `par` [mas]
            # 09 par_m: Minus value of `par` [mas]
            # 10 v_lsr: Local Standard of Rest velocity [km/s]
            # 11 v_lsr_e: Uncertainty in `v_lsr` [km/s]
            # 12 v_lsr_p: Plus value of `v_lsr` [km/s]
            # 13 v_lsr_m: Minus value of `v_lsr` [km/s]
            # 14 mu_x: Eastward proper motion [mas/yr]
            # 15 mu_x_e: Uncertainty in `mu_x` [mas/yr]
            # 16 mu_x_p: Plus value of `mu_x` [mas/yr]
            # 17 mu_x_m: Minus value of `mu_x` [mas/yr]
            # 18 mu_y: Northward proper motion [mas/yr]
            # 19 mu_y_e: Uncertainty in `mu_y` [mas/yr]
            # 20 mu_y_p: Plus value of `mu_y` [mas/yr]
            # 21 mu_y_m: Minus value of `mu_y` [mas/yr]
            # 22 r: Heliocentric distance [kpc]
            # 23 r_ep: Plus uncertainty in `r` [kpc]
            # 24 r_em: Minus uncertainty in `r` [kpc]
            # 25 r_p: Plus value of `r` [kpc]
            # 26 r_m: Minus value of `r` [kpc]
            # 27 l: Longitude [radians]
            # 28 b: Latitude [radians]
            # 29 mu_l: Proper motion in longitude [mas/yr]
            # 30 mu_b: Proper motion in latitude [mas/yr]
            # 31 R: Galactocentric distance [kpc]
            # 32 R_ep: Plus uncertainty in `R` [kpc]
            # 33 R_em: Minus uncertainty in `R` [kpc]
            # 34 R_p: Plus value of `R` [kpc]
            # 35 R_m: Minus value of `R` [kpc]
            # 36 X: X coordinate [kpc]
            # 37 X_ep: Plus uncertainty in `X` [kpc]
            # 38 X_em: Minus uncertainty in `X` [kpc]
            # 39 X_p: Plus value of `X` [kpc]
            # 40 X_m: Minus value of `X` [kpc]
            # 41 Y: Y coordinate [kpc]
            # 42 Y_ep: Plus uncertainty in `Y` [kpc]
            # 43 Y_em: Minus uncertainty in `Y` [kpc]
            # 44 Y_p: Plus value of `Y` [kpc]
            # 45 Y_m: Minus value of `Y` [kpc]
            # 46 Z: Z coordinate [kpc]
            # 47 Z_ep: Plus uncertainty in `Z` [kpc]
            # 48 Z_em: Minus uncertainty in `Z` [kpc]
            # 49 Z_p: Plus value of `Z` [kpc]
            # 50 Z_m: Minus value of `Z` [kpc]
            # 51 theta: Azimuthal velocity [km/s]
            # 52 theta_ep: Plus uncertainty in `theta` [km/s]
            # 53 theta_em: Minus uncertainty in `theta` [km/s]
            # 54 theta_p: Plus value of `theta` [km/s]
            # 55 theta_m: Minus value of `theta` [km/s]
            # 56 theta_evel: Velocity uncertainty in `theta` [km/s]
            # 57 V_r: Heliocentric velocity in distance [km/s]
            # 58 V_l: Heliocentric velocity in longitude [km/s]
            # 59 V_l_ep: Plus uncertainty in `V_l` [km/s]
            # 60 V_l_em: Minus uncertainty in `V_l` [km/s]
            # 61 V_l_p: Plus value of `V_l` [km/s]
            # 62 V_l_m: Minus value of `V_l` [km/s]
            # 63 V_b: Heliocentric velocity in latitude [km/s]
            # 64 V_b_ep: Plus uncertainty in `V_b` [km/s]
            # 65 V_b_em: Minus uncertainty in `V_b` [km/s]
            # 66 V_b_p: Plus value of `V_b` [km/s]
            # 67 V_b_m: Minus value of `V_b` [km/s]
            # 68 U: U coordinate [km/s]
            # 69 U_ep: Plus uncertainty in `U` [km/s]
            # 70 U_em: Minus uncertainty in `U` [km/s]
            # 71 U_p: Plus value of `U` [km/s]
            # 72 U_m: Minus value of `U` [km/s]
            # 73 V: V coordinate [km/s]
            # 74 V_ep: Plus uncertainty in `V` [km/s]
            # 75 V_em: Minus uncertainty in `V` [km/s]
            # 76 V_p: Plus value of `V` [km/s]
            # 77 V_m: Minus value of `V` [km/s]
            # 78 W: W coordinate [km/s]
            # 79 W_ep: Plus uncertainty in `W` [km/s]
            # 80 W_em: Minus uncertainty in `W` [km/s]
            # 81 W_p: Plus value of `W` [km/s]
            # 82 W_m: Minus value of `W` [km/s]
            #
            # Uncertainties come from assuming maximum and minimum values of the parallax.
            # Note that they are not independent from each other and can be negative here.
            #
            # Parameters used:
            #
            # The right ascension of the north galactic pole [HMS angle -> radians]
            # Source: Reid et al. (2009)
            # ALPHA_NGP: {alpha_ngp} [12:51:26.2817]
            #
            # The declination of the north galactic pole [DMS angle -> radians]
            # Source: Reid et al. (2009)
            # DELTA_NGP: {delta_ngp} [27:07:42.013]
            #
            # Linear velocities units conversion coefficient
            # Sources: Gromov, Nikiforov (2016)
            # K: {k}
            #
            # The longitude of the north celestial pole [decimal degrees angle -> radians]
            # Source: Reid et al. (2009)
            # L_NCP: {l_ncp} [122.932]
            #
            # Galactocentric distance to the Sun [kpc]
            # R_0: {r_0}
            #
            # Full circular velocity of the Sun [km/s]
            # Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
            # THETA_SUN: {theta_sun}
            #
            # Peculiar motion locally toward GC [km/s]
            # Sources: Reid et al. (2019); Gromov, Nikiforov (2021)
            # U_SUN: {u_sun}
            #
            # Standard Solar Motion toward GC [km/s]
            # Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
            # U_SUN_STANDARD: {u_sun_standard}
            #
            # Standard Solar Motion toward l = 90 degrees [km/s]
            # Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
            # V_SUN_STANDARD: {v_sun_standard}
            #
            # Standard Solar Motion toward NGP [km/s]
            # Sources: Reid et al. (2009); Gromov, Nikiforov (2021)
            # W_SUN_STANDARD: {w_sun_standard}
            #
            ",
            sample_description = self.format_sample_description(),
            alpha_ngp = self.params.alpha_ngp,
            delta_ngp = self.params.delta_ngp,
            k = self.params.k,
            l_ncp = self.params.l_ncp,
            r_0 = self.params.r_0,
            theta_sun = self.params.theta_sun,
            u_sun = self.params.u_sun,
            u_sun_standard = self.params.u_sun_standard,
            v_sun_standard = self.params.v_sun_standard,
            w_sun_standard = self.params.w_sun_standard,
        );
        let records = &self.objects;
        output::serialize_to(dat_dir, bin_dir, "objects", &header, records)
    }
}
