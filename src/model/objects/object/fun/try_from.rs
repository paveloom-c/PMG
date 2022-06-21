//! Get an object from the input record

use super::super::coordinates::EquatorialSpherical;
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
        // Initialize an empty object
        let mut object = Self::default();
        // Unpack the data into the object
        object.equatorial_s.replace(
            EquatorialSpherical::try_from(&record)
                .with_context(|| "Couldn't parse the equatorial coordinates")?,
        );
        object.name.replace(record.name);
        object.obj_type.replace(record.obj_type);
        object.source.replace(record.source);
        Ok(object)
    }
}
