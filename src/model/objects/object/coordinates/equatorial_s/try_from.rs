//! Parse values from the record and push them to the storage

use super::super::EquatorialSpherical;

use std::error::Error;
use std::str::FromStr;

use crate::model::io::input;
use crate::utils::{dms2rad, hms2rad, str2vec};

use anyhow::{bail, Context, Result};
use num::Float;

impl<F> TryFrom<&input::Record<F>> for EquatorialSpherical<F>
where
    F: Float + FromStr,
    <F as FromStr>::Err: Error + Send + Sync + 'static,
{
    type Error = anyhow::Error;

    fn try_from(record: &input::Record<F>) -> Result<Self> {
        // Parse the right ascension string and convert the angle to radians
        let alpha = match str2vec(&record.alpha)
            .with_context(|| format!("Couldn't parse the string {:?}", &record.alpha))?[..]
        {
            [hours, minutes, seconds] => hms2rad(hours, minutes, seconds),
            _ => bail!("Three values were expected"),
        };
        // Parse the declination string and convert the angle to radians
        let delta = match str2vec(&record.delta)
            .with_context(|| format!("Couldn't parse the string {:?}", &record.delta))?[..]
        {
            [degrees, minutes, seconds] => dms2rad(degrees, minutes, seconds),
            _ => bail!("Three values were expected"),
        };
        // Get the parallax
        let par = record.par;
        Ok(Self { alpha, delta, par })
    }
}
