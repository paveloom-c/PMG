//! Rotation curve

use crate::model::{Model, Object};

use std::fmt::Debug;

use anyhow::{Context, Result};
use indoc::indoc;
use num::Float;
use serde::Serialize;

/// Header of the `rotcurve.dat` file
pub(in crate::model) const ROTCURVE_CSV_HEADER: &str = indoc! {"
    # Rotation curve
    #
    # Descriptions:
    #
    # 1 name: Name of the object
    # 2 theta: Azimuthal velocity (km/s)
    # 3 R: Galactocentric distance (kpc)
    # 4 type: Type of the object
    # 5 source: Source of the data
    #\n
"};

/// Output data record
#[derive(Serialize)]
pub(in crate::model) struct Record<'a, F: Float + Debug> {
    /// Name
    pub(in crate::model) name: &'a String,
    /// Azimuthal velocity (km/s)
    pub(in crate::model) theta: F,
    /// Galactocentric distance (kpc)
    #[serde(rename = "R")]
    pub(in crate::model) r_g: F,
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
        let (theta, r_g) = object.rotation_c()?.into();
        let obj_type = object.obj_type()?;
        let source = object.source()?;
        Ok(Self {
            name,
            theta,
            r_g,
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
