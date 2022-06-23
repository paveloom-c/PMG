//! Galactic heliocentric coordinates in the Cartesian system

use crate::model::{Model, Object};

use std::fmt::Debug;

use anyhow::{Context, Result};
use indoc::indoc;
use num::Float;
use serde::Serialize;

/// Header of the `coords.dat` file
pub(in crate::model) const COORDS_CSV_HEADER: &str = indoc! {"
    # Galactic coordinates of the objects
    #
    # Descriptions:
    #
    # 1  name: Name of the object
    # 2  l: Longitude [deg]
    # 3  b: Latitude [deg]
    # 4  X: X coordinate [kpc]
    # 5  ep_X: Plus uncertainty in `X` [kpc]
    # 6  em_X: Minus uncertainty in `X` [kpc]
    # 7  Y: Y coordinate [kpc]
    # 8  ep_Y: Plus uncertainty in `Y` [kpc]
    # 9  em_Y: Minus uncertainty in `Y` [kpc]
    # 10 Z: Z coordinate [kpc]
    # 11 ep_Z: Plus uncertainty in `Z` [kpc]
    # 12 em_Z: Minus uncertainty in `Z` [kpc]
    # 13 r: Heliocentric distance [kpc]
    # 14 ep_r: Plus uncertainty in `r` [kpc]
    # 15 em_r: Minus uncertainty in `r` [kpc]
    # 16 R: Galactocentric distance [kpc]
    # 17 ep_R: Plus uncertainty in `R` [kpc]
    # 18 em_R: Minus uncertainty in `R` [kpc]
    # 19 type: Type of the object
    # 20 source: Source of the data
    #\n
"};

/// Output data record
#[derive(Serialize)]
pub(in crate::model) struct Record<'a, F: Float + Debug> {
    /// Name
    pub(in crate::model) name: &'a String,
    /// Longitude (deg)
    pub(in crate::model) l: F,
    /// Latitude (deg)
    pub(in crate::model) b: F,
    /// X coordinate (kpc)
    #[serde(rename = "X")]
    pub(in crate::model) x: F,
    /// Plus uncertainty in `x` (kpc)
    #[serde(rename = "ep_X")]
    pub(in crate::model) e_p_x: F,
    /// Minus uncertainty in `x` (kpc)
    #[serde(rename = "em_X")]
    pub(in crate::model) e_m_x: F,
    /// Y coordinate (kpc)
    #[serde(rename = "Y")]
    pub(in crate::model) y: F,
    /// Plus uncertainty in `y` (kpc)
    #[serde(rename = "ep_Y")]
    pub(in crate::model) e_p_y: F,
    /// Minus uncertainty in `y` (kpc)
    #[serde(rename = "em_Y")]
    pub(in crate::model) e_m_y: F,
    /// Z coordinate (kpc)
    #[serde(rename = "Z")]
    pub(in crate::model) z: F,
    /// Plus uncertainty in `z` (kpc)
    #[serde(rename = "ep_Z")]
    pub(in crate::model) e_p_z: F,
    /// Minus uncertainty in `z` (kpc)
    #[serde(rename = "em_Z")]
    pub(in crate::model) e_m_z: F,
    /// Heliocentric distance (kpc)
    #[serde(rename = "r")]
    pub(in crate::model) r_h: F,
    /// Plus uncertainty in `r_h` (kpc)
    #[serde(rename = "ep_r")]
    pub(in crate::model) e_p_r_h: F,
    /// Minus uncertainty in `r_h` (kpc)
    #[serde(rename = "em_r")]
    pub(in crate::model) e_m_r_h: F,
    /// Galactocentric distance (kpc)
    #[serde(rename = "R")]
    pub(in crate::model) r_g: F,
    /// Plus uncertainty in `r_g` (kpc)
    #[serde(rename = "ep_R")]
    pub(in crate::model) e_p_r_g: F,
    /// Minus uncertainty in `r_g` (kpc)
    #[serde(rename = "em_R")]
    pub(in crate::model) e_m_r_g: F,
    /// Type of the object
    #[serde(rename = "type")]
    pub(in crate::model) obj_type: &'a String,
    /// Source of the data
    pub(in crate::model) source: &'a String,
}

#[allow(clippy::many_single_char_names)]
impl<'a, F: Float + Debug> TryFrom<&'a Object<F>> for Record<'a, F> {
    type Error = anyhow::Error;

    fn try_from(object: &'a Object<F>) -> Result<Self> {
        let name = object.name()?;
        let (l, b) = object.galactic_s()?.into();
        let (x, y, z) = object.galactic_c()?.into();
        let (r_h, r_g) = object.distances()?.into();
        let obj_type = object.obj_type()?;
        let source = object.source()?;
        Ok(Self {
            name,
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
            obj_type,
            source,
        })
    }
}

/// Output data records
pub(in crate::model) type Records<'a, F> = Vec<Record<'a, F>>;

impl<'a, F: Float + Debug> TryFrom<&'a Model<F>> for Records<'a, F> {
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
