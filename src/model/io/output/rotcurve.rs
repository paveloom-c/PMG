//! Rotation curve

use crate::model::{Model, Object};

use std::fmt::Debug;
use std::path::Path;

use anyhow::{Context, Result};
use indoc::indoc;
use num::Float;
use serde::Serialize;

/// Name of the files
const NAME: &str = "rotcurve";

/// Header of the text file
const HEADER: &str = indoc! {"
    # Rotation curve
    #
    # Descriptions:
    #
    # 1 name: Name of the object
    # 2 theta: Azimuthal velocity [km/s]
    # 3 e_theta: Uncertainty in `theta` [km/s]
    # 4 R: Galactocentric distance [kpc]
    # 5 ep_R: Plus uncertainty in `R` [kpc]
    # 6 em_R: Minus uncertainty in `R` [kpc]
    # 7 type: Type of the object
    # 8 source: Source of the data
    #
    # Uncertainties in the distance come from assuming maximum and minimum
    # values of the parallax. Note that they can be negative here.
    #\n
"};

/// Output data record
#[derive(Serialize)]
struct Record<'a, F: Float + Debug> {
    /// Name
    name: &'a String,
    /// Azimuthal velocity (km/s)
    theta: F,
    /// Uncertainty in `theta` (km/s)
    e_theta: F,
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
impl<'a, F: Float + Debug> TryFrom<&'a Object<F>> for Record<'a, F> {
    type Error = anyhow::Error;

    fn try_from(object: &'a Object<F>) -> Result<Self> {
        let name = object.name()?;
        let (theta, r_g) = object.rotation_c()?.into();
        let obj_type = object.obj_type()?;
        let source = object.source()?;
        Ok(Self {
            name,
            theta: theta.v,
            e_theta: theta.e_p,
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

/// Serialize records to the files
pub(in crate::model) fn serialize_to<F>(
    dat_dir: &Path,
    bin_dir: &Path,
    model: &Model<F>,
) -> Result<()>
where
    F: Float + Debug + Serialize,
{
    super::serialize_to(
        dat_dir,
        bin_dir,
        NAME,
        HEADER,
        Records::try_from(model).with_context(|| "Couldn't construct records from the model")?,
    )
}
