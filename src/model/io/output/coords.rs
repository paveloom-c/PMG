//! Galactic heliocentric coordinates in the Cartesian system

use crate::model::{Model, Object};

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
    # name: Name of the object
    # l: Longitude [deg]
    # b: Latitude [deg]
    # x: X coordinate [kpc]
    # y: Y coordinate [kpc]
    # z: Z coordinate [kpc]
    # r_h: Heliocentric distance [kpc]
    # r_g: Galactocentric distance [kpc]
    # obj_type: Type of the object
    # source: Source of the data
    #\n
"};

/// Output data record
#[derive(Serialize)]
pub(in crate::model) struct Record<'a, F: Float> {
    /// Name
    pub(in crate::model) name: &'a String,
    /// Longitude (deg)
    pub(in crate::model) l: F,
    /// Latitude (deg)
    pub(in crate::model) b: F,
    /// X coordinate (kpc)
    pub(in crate::model) x: F,
    /// Y coordinate (kpc)
    pub(in crate::model) y: F,
    /// Z coordinate (kpc)
    pub(in crate::model) z: F,
    /// Heliocentric distance (kpc)
    pub(in crate::model) r_h: F,
    /// Galactocentric distance (kpc)
    pub(in crate::model) r_g: F,
    /// Type of the object
    pub(in crate::model) obj_type: &'a String,
    /// Source of the data
    pub(in crate::model) source: &'a String,
}

#[allow(clippy::many_single_char_names)]
impl<'a, F: Float> TryFrom<&'a Object<F>> for Record<'a, F> {
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
            x,
            y,
            z,
            r_h,
            r_g,
            obj_type,
            source,
        })
    }
}

/// Output data records
pub(in crate::model) type Records<'a, F> = Vec<Record<'a, F>>;

impl<'a, F: Float> TryFrom<&'a Model<F>> for Records<'a, F> {
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
