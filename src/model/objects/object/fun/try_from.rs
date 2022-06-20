//! Get an object from the input record

use super::super::coordinates::{EquatorialSpherical, GalacticCartesian};
use super::super::Object;
use crate::model::io::input;

use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{Context, Result};
use num::Float;

impl<F> TryFrom<input::Record<F>> for Object<F>
where
    F: Float + Debug + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    fn try_from(record: input::Record<F>) -> Result<Self> {
        // Unpack the data
        let equatorial_s = EquatorialSpherical::try_from(&record)
            .with_context(|| "Couldn't parse the equatorial coordinates")?;
        let galactic_c = GalacticCartesian::from(&equatorial_s);
        let name = record.name;
        let obj_type = record.obj_type;
        let source = record.source;
        Ok(Self {
            name,
            equatorial_s,
            galactic_c,
            obj_type,
            source,
        })
    }
}
